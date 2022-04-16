extern crate rcgen;

use rcgen::generate_simple_self_signed;
use std::fs;

pub fn new_certificate(ip: String, port: u16) {
	let subject_alt_names: &[_] = &[ip.clone() + ":" + &port.to_string()];

	let cert = generate_simple_self_signed(subject_alt_names).unwrap();
	println!(
		"Not generating certificate, using hard-coded mktemp path.: {}",
		ip
	);
	fs::write("/tmp/warpy.pem", cert.serialize_pem().unwrap()).expect("Cannot write PEM file.");
	fs::write("/tmp/warpy.key", cert.serialize_private_key_pem()).expect("Cannot write KEY file.");
}
