use std::io::Write;
use std::fs::File;

fn make_module(file_name: &str) {
    let magic: [u8; 4] = [0x6D, 0x73, 0x61, 0x00];
    let version: [u8; 4] = [0x00, 0x00, 0x00, 0x01];

    let mut f = File::create(file_name).unwrap();
    let _ = f.write_all(&magic);
    let _ = f.write_all(&version);
}

fn validate_module(module: Module) {
    let context = Context::new(module.types);
}

fn decode_wasm(file_name: &str) {
    let mut f = File::open(file_name).unwrap();
    let mut reader = BufReader::new(f);
    let _  = read_module(&mut reader);
}

fn instanciate() {}

fn invoke() {}