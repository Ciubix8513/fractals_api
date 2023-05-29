#![allow(
    clippy::unused_async,
    clippy::wildcard_imports,
    clippy::future_not_send
)]
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use actix_web::{middleware, App, HttpServer};
use dotenvy::dotenv;
use utils::graphics::{get_device, PipelineBufers};

use crate::endpoints::*;

mod endpoints;
mod grimoire;
mod utils;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Fractals {
    Mandebrot,
    Custom(String),
}
pub type PipelineStore = Mutex<HashMap<Fractals, PipelineBufers>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = env::var("IP_ADDRESS").expect("Ip adress should be set");
    let port = env::var("PORT")
        .expect("Port must be set")
        .parse()
        .expect("Invalid port number");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(main_page)
            .data_factory(|| async { get_device().await })
            .app_data(actix_web::web::Data::new(
                PipelineStore::new(HashMap::new()),
            ))
            .service(render_image)
            .wrap(middleware::Logger::default())
    })
    .bind((ip, port))?
    .run()
    .await
}

#[actix_web::test]
async fn renderer_test() {
    let mut app = actix_web::test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(
                PipelineStore::new(HashMap::new()),
            ))
            .data_factory(|| async { get_device().await })
            .service(render_image),
    )
    .await;
    let req = actix_web::test::TestRequest::with_uri("/test").to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
}
