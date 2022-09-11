use std::{net::SocketAddr, str::FromStr};

use axum::{response::IntoResponse, Json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/", axum::routing::get(get_root))
        .route("/json", axum::routing::get(get_json));

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_root() -> impl IntoResponse {
    "hello"
}

async fn get_json() -> impl IntoResponse {
    Json("hello")
}
