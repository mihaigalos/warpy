extern crate rcgen;

use rcgen::generate_simple_self_signed;
use std::fs;

use crate::constants::*;

pub fn new_certificate(ip: String, port: u16) {
	let subject_alt_names: &[_] = &[ip + ":" + &port.to_string()];

	let cert = generate_simple_self_signed(subject_alt_names).unwrap();

	fs::write(PEM_FILE, cert.serialize_pem().unwrap()).expect("Cannot write PEM file.");
	fs::write(KEY_FILE, cert.serialize_private_key_pem()).expect("Cannot write KEY file.");
}
