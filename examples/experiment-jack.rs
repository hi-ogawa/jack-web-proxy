use std::io::Write;

use jack::PortFlags;

fn main() {
    // create two client
    let (client, _status) = jack::Client::new(
        "rust_jack_patchbay_notifier",
        jack::ClientOptions::NO_START_SERVER,
    )
    .unwrap();

    let (client_controller, _status) = jack::Client::new(
        "rust_jack_patchbay_controller",
        jack::ClientOptions::NO_START_SERVER,
    )
    .unwrap();

    {
        // get all ports
        let port_names = client.ports(None, None, PortFlags::empty());
        let ports: Vec<_> = port_names
            .iter()
            .map(|name| client.port_by_name(name))
            .flatten()
            .collect();
        dbg!(&ports);

        // get all connections from each output
        for port in ports {
            if port.flags().intersects(PortFlags::IS_OUTPUT) {
                unsafe {
                    let connections = collect_strs(jack_sys::jack_port_get_all_connections(
                        client.raw(),
                        port.raw(),
                    ));
                    dbg!((port.name().unwrap(), &connections));
                }
            }
        }
    }

    // setup callback for port/connection changes
    let handler = PortChangeNotifier(|| {
        println!("notified");
    });
    let active_client = client.activate_async(handler, ()).unwrap();

    // command line loop
    loop {
        print!("command > ");
        std::io::stdout().flush().unwrap();
        match get_line() {
            Ok(command) => {
                if command.trim() == "q" {
                    break;
                }
                match command.trim() {
                    "q" | "quit" => break,
                    "connect" => {
                        print!("output > ");
                        std::io::stdout().flush().unwrap();
                        match get_line() {
                            Ok(output) => {
                                print!("input > ");
                                std::io::stdout().flush().unwrap();
                                match get_line() {
                                    Ok(input) => {
                                        client_controller
                                            .connect_ports_by_name(&output.trim(), &input.trim())
                                            .unwrap();
                                    }
                                    Err(_) => continue,
                                }
                            }
                            Err(_) => continue,
                        }
                    }
                    "disconnect" => {
                        print!("in < ");
                        print!("out > ");
                    }
                    _ => continue,
                }
            }
            Err(_) => break,
        }
    }

    active_client.deactivate().unwrap();
}

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

//
// https://github.com/RustAudio/rust-jack/blob/6133ac4d3654668f461a036fef8e3d9655b6c372/src/jack_utils.rs
//

use jack_sys as j;
use std::ffi;

pub unsafe fn collect_strs(ptr: *const *const libc::c_char) -> Vec<String> {
    if ptr.is_null() {
        return Vec::new();
    };
    let len = {
        let mut len = 0;
        while !(*ptr.offset(len)).is_null() {
            len += 1;
        }
        len
    };
    let mut strs = Vec::with_capacity(len as usize);
    for i in 0..len {
        let cstr_ptr = *ptr.offset(i);
        let s = ffi::CStr::from_ptr(cstr_ptr).to_string_lossy().into_owned();
        strs.push(s);
    }
    j::jack_free(ptr as *mut ::libc::c_void);
    strs
}

// misc

fn get_line() -> std::io::Result<String> {
    let mut command = String::new();
    std::io::stdin().read_line(&mut command)?;
    Ok(command)
}
