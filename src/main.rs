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

use std::env;

fn main() {
    println!("{:?}", "polytope");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args[1] == "kick" {
        let _ = kick();
    }
}
