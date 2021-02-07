use std::env;
use polytope::*;

fn main() {
    println!("{}", "polytope");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args[1].as_ref() {
        "kick" => {
            let _ = kick();
        },
        "module" => make_module("a.wasm"),
        // "test" => test_rotate_shift(),
        "decode" => decode_wasm("hobogo.wasm"),
        _ => println!("invalid sub command"),
    } 
}
