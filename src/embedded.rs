use std::io::Write;
use std::io::BufReader;
use std::fs::File;
use crate::module::read_module;

pub fn make_module(file_name: &str) {
    let magic: [u8; 4] = [0x6D, 0x73, 0x61, 0x00];
    let version: [u8; 4] = [0x00, 0x00, 0x00, 0x01];

    let mut f = File::create(file_name).unwrap();
    let _ = f.write_all(&magic);
    let _ = f.write_all(&version);
}

pub fn decode_wasm(file_name: &str) {
    let mut f = File::open(file_name).unwrap();
    let mut reader = BufReader::new(f);
    let _  = read_module(&mut reader);
}

fn instanciate() {}

fn invoke() {}