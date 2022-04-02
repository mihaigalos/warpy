use std::net::TcpListener;
use std::ops::Range;

pub fn next_port_in_range(port_range: Range<u16>) -> Option<u16> {
    for port in port_range {
        if is_port_available(port) {
            return Some(port);
        }
    }
    None
}

fn is_port_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
