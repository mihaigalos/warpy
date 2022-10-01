use std::net::TcpListener;
use std::ops::Range;

pub fn next_port_in_range(mut port_range: Range<u16>) -> Option<u16> {
    port_range.find(|&port| is_port_available(port))
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}
