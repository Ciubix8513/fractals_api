#![allow(clippy::unused_async)]
use std::env;

use actix_web::{App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

#[actix_web::get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = env::var("IP_ADDRESS").expect("Ip adress should be set");
    let port = env::var("PORT")
        .expect("Port must be set")
        .parse()
        .expect("Invalid port number");

    HttpServer::new(move || App::new().service(main_page))
        .bind((ip, port))?
        .run()
        .await
}
