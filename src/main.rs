use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello Polytope!";

    // HttpResponse::Ok() はステータスコード 200 を持つ HttpResponseBuilder という構造体を返します。
    // HttpResponseBuilder の body() という関数にレスポンスのボディを渡すと HttpResponse が返ってきます。
    // 戻り値の型が Result なので Ok で包みます。
    Ok(HttpResponse::Ok().body(response_body))
}

#[get("/a")]
async fn index2() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[actix_rt::main]
async fn kick() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index2))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

use std::io::Write;

fn make_module(file_name: &str) {
    let magic: [u8; 4] = [0x6D, 0x73, 0x61, 0x00];
    let version: [u8; 4] = [0x00, 0x00, 0x00, 0x01];
    use std::fs::File;
    let mut f = File::create(file_name).unwrap();
    let _ = f.write_all(&magic);
    let _ = f.write_all(&version);
}

type Byte = u8;
use std::io::{Read, BufReader};

fn read_u32_from_leb128<T: Read>(reader: &mut BufReader<T>) -> u32 {
    let mut acc: u32 = 0;
    let mut count: u8 = 0;
    for byte in reader.bytes() {
        if let Ok(b) = byte {
            let val: u32 = (b & 0b01111111) as u32;
            let shifted_val = val << (7 * count);
            acc += shifted_val as u32;
            count += 1;
            if b < 0b10000000 { break; }
        } else {
            break;
        }
    }
    acc
}

#[test]
fn test_read_u32_from_leb128() {
    let data: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    let mut reader = BufReader::new(data.as_ref());
    let res = read_u32_from_leb128(&mut reader);
    println!("{:x?}", res);
}

/// 1バイトを最初の1bitとそれ以外に分ける
enum LEB128Flag {
    Zero,
    One,
}

fn split_byte(b: Byte) -> (LEB128Flag, u8) {
    if b < 0b10000000 {
        (LEB128Flag::Zero, b)
    } else {
        (LEB128Flag::One, b - 0b10000000)
    }
}

// fn test_module(file_name: &str) {
//     let magic: [u8; 4] = [0x6D, 0x73, 0x61, 0x00];
//     let version: [u8; 4] = [0x00, 0x00, 0x00, 0x01];
//     use std::fs::File;
//     let mut f = File::create(file_name).unwrap();
//     let _ = f.write_all(&magic);
//     let _ = f.write_all(&version);
// }

struct FuncType {

}
struct Module {
    types: Vec<FuncType>, 
}

struct Context {
    types: Vec<FuncType>,
    funcs: (),
    tables: (),
    mems: (),
    globals: (),
    locals: (),
    labels: (),
    r#return: (),
}

impl Context {
    fn new(types: Vec<FuncType>) -> Self {
        Context {
            types: types,
            funcs: (),
            tables: (),
            mems: (),
            globals: (),
            locals: (),
            labels: (),
            r#return: (),
        }
    }
}
fn validate_module(module: Module) {
    let context = Context::new(module.types);
}

fn instanciate() {

}

fn invoke() {

}

#[allow(overflowing_literals)]
fn test_rotate_shift() {
    // let n = 0xaa00000000006e1u64;
    // let n = 0x1234567890ABCDEFu64;
    // // let m = 0x6e10aa;
    // println!("{:x}", n.rotate_left(8));
    // assert_eq!(n.rotate_left(1), m);
    let o = 0b11110000u8;
    assert_eq!(o.rotate_left(3), 0b10000111);
    println!("{:b}", o.rotate_left(3));
    assert_eq!(o << 3, 0b10000000);
    
    let p = 0b10000000i8;
    println!("{:8b}", p.rotate_right(1));
    println!("{:8b}", p >> 1);
    assert_eq!(p.rotate_right(1), 0b01000000);
    assert_eq!(p >> 1, 0b11000000);

    // let data: [u8; 4] = [0b11001100, 0b01000011, 86, 120];
    let data: [u8; 4] = [0xE5, 0x8E, 0x26, 120];
    let mut reader = BufReader::new(data.as_ref());
    let res = read_u32_from_leb128(&mut reader);
    println!("res: {:b}", res);
    println!("res: {}", res);
}

use std::env;

fn main() {
    println!("{}", "polytope");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args[1].as_ref() {
        "kick" => {
            let _ = kick();
        },
        "module" => make_module("a.wasm"),
        "test" => test_rotate_shift(),
        _ => println!("invalid sub command"),
    } 
}
