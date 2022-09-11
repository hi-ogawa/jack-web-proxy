use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::Event;
use axum::response::sse::{KeepAlive, Sse};
use axum::routing::{get, post};
use axum::{response::IntoResponse, Json};
use futures::stream::Stream;
use jack::PortFlags;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use std::{net::SocketAddr, str::FromStr};
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::RwLock;
mod utils;

// TODO: logging

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //
    // two jack clients
    //
    let (jack_client_notification, _status) = jack::Client::new(
        "jack-web-proxy-notification",
        jack::ClientOptions::NO_START_SERVER,
    )?;

    let (jack_client_handler, _status) = jack::Client::new(
        "jack-web-proxy-handler",
        jack::ClientOptions::NO_START_SERVER,
    )?;

    //
    // broadcast channel to notify change via SSE
    //
    let (notification_sender, _) = broadcast::channel::<()>(1000);

    let app_state = Arc::new(RwLock::new(AppState {
        jack_client: jack_client_handler,
        notification_sender: notification_sender.clone(),
    }));

    //
    // start jack notification callback thread
    //
    let active_client = jack_client_notification.activate_async(
        PortChangeNotifier(move || {
            println!("send result = {:?}", notification_sender.send(()));
        }),
        (),
    )?;

    //
    // define route
    //
    let app = axum::Router::with_state(app_state)
        .route("/", get(get_root))
        .route("/sse", get(get_sse))
        .route("/summary", get(get_summary))
        // TODO: auto generate jack APIs
        .route("/api/connect", post(post_api_connect))
        .route("/api/disconnect", post(post_api_disconnect));

    //
    // run app
    //
    let addr = SocketAddr::from_str("127.0.0.1:3000")?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    active_client.deactivate()?;
    Ok(())
}

type AppStateWrapper = Arc<RwLock<AppState>>;

struct AppState {
    jack_client: jack::Client,
    notification_sender: Sender<()>,
}

//
// GET /
//

async fn get_root() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

//
// GET /summary
//

struct SerdePortFlags(PortFlags);

impl Serialize for SerdePortFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(
            [
                PortFlags::IS_INPUT,
                PortFlags::IS_OUTPUT,
                PortFlags::IS_PHYSICAL,
                PortFlags::IS_TERMINAL,
                PortFlags::CAN_MONITOR,
            ]
            .into_iter()
            .filter(|flag| flag.intersects(self.0))
            .map(|flag| format!("{:?}", flag)),
        )
    }
}

#[derive(serde::Serialize)]
struct PortInfo {
    id: String,
    // TODO
    // group_name: String,
    // port_name: String,
    flags: SerdePortFlags,
}

#[derive(serde::Serialize)]
struct ConnectionInfo {
    source: String,
    destinations: Vec<String>,
}

#[derive(serde::Serialize)]
struct AllInfo {
    ports: Vec<PortInfo>,
    connections: Vec<ConnectionInfo>,
}

async fn get_summary(State(state): State<AppStateWrapper>) -> impl IntoResponse {
    let jack_client = &state.read().await.jack_client;

    // get all ports
    let port_names = jack_client.ports(None, None, jack::PortFlags::empty());

    let mut result = AllInfo {
        ports: vec![],
        connections: vec![],
    };

    for id in &port_names {
        if let Some(port) = jack_client.port_by_name(id) {
            let flags = port.flags();

            result.ports.push(PortInfo {
                id: id.clone(),
                flags: SerdePortFlags(flags),
            });

            // get connections from each output
            if flags.intersects(jack::PortFlags::IS_OUTPUT) {
                unsafe {
                    let destinations = utils::collect_c_strings(
                        jack_sys::jack_port_get_all_connections(jack_client.raw(), port.raw()),
                    );
                    result.connections.push(ConnectionInfo {
                        source: id.clone(),
                        destinations,
                    });
                }
            }
        }
    }

    Json(result)
}

//
// GET /sse
//

async fn get_sse(
    State(state): State<AppStateWrapper>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let tx = &mut state.write().await.notification_sender;
    let mut rx = tx.subscribe();

    // https://docs.rs/tokio/1.0.0/tokio/stream/index.html#example
    let stream = async_stream::stream! {
        while let Ok(_) = rx.recv().await {
            yield Ok(axum::response::sse::Event::default().data("changed"));
        }
    };

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keepalive"),
    )
}

//
// POST /api/connect
//

#[derive(Deserialize)]
struct ConnectionRequest {
    source: String,
    destination: String,
}

async fn post_api_connect(
    State(state): State<AppStateWrapper>,
    Json(req): Json<ConnectionRequest>,
) -> impl IntoResponse {
    let jack_client = &mut state.write().await.jack_client;
    match jack_client.connect_ports_by_name(&req.source, &req.destination) {
        Err(e) => Err(Json(json!({ "success": false, "error": e.to_string() }))),
        _ => Ok(Json(json!({ "success": true }))),
    }
}

//
// POST /api/disconnect
//

async fn post_api_disconnect(
    State(state): State<AppStateWrapper>,
    Json(req): Json<ConnectionRequest>,
) -> impl IntoResponse {
    let jack_client = &mut state.write().await.jack_client;
    match jack_client.disconnect_ports_by_name(&req.source, &req.destination) {
        Err(e) => Err(Json(json!({ "success": false, "error": e.to_string() }))),
        _ => Ok(Json(json!({ "success": true }))),
    }
}

//
// jack notification closure callback
//

struct PortChangeNotifier<F: 'static + Send + Fn()>(F);

impl<F: 'static + Send + Fn()> jack::NotificationHandler for PortChangeNotifier<F> {
    fn port_registration(
        &mut self,
        _: &jack::Client,
        _port_id: jack::PortId,
        _is_registered: bool,
    ) {
        (self.0)();
    }

    fn port_rename(
        &mut self,
        _: &jack::Client,
        _port_id: jack::PortId,
        _old_name: &str,
        _new_name: &str,
    ) -> jack::Control {
        (self.0)();
        jack::Control::Continue
    }

    fn ports_connected(
        &mut self,
        _: &jack::Client,
        _port_id_a: jack::PortId,
        _port_id_b: jack::PortId,
        _are_connected: bool,
    ) {
        (self.0)();
    }
}
