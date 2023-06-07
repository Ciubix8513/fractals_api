#![allow(
    clippy::unused_async,
    clippy::wildcard_imports,
    clippy::future_not_send
)]
use std::env;
use std::sync::Mutex;

use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use dotenvy::dotenv;
use structs::{rendering::PipelineStore, requests::RequestIdentifier};
use utils::graphics::generate_backend;

use crate::endpoints::*;

///Module to contain all the endpoints
mod endpoints;
///Module to store all the magic values
mod grimoire;
///Structure/Enum definitions
mod structs;
///Various utility functions
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = env::var("IP_ADDRESS").expect("Ip adress should be set");
    let port = env::var("PORT")
        .expect("Port must be set")
        .parse()
        .expect("Invalid port number");
    let debug = env::var("DEBUG").is_ok();

    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env_logger::Builder::new()
        .filter_module(
            grimoire::LOGGING_TARGET,
            if debug {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            },
        )
        .filter_module("actix_web", log::LevelFilter::Info)
        .filter_module("actix_server", log::LevelFilter::Info)
        .filter_module("wgpu", log::LevelFilter::Off)
        .format_timestamp(None)
        .init();

    HttpServer::new(move || {
        App::new()
            .service(main_page)
            .service(coloring_page)
            .data_factory(|| async { generate_backend().await })
            .app_data(Data::new(Mutex::new(
                Vec::<(RequestIdentifier, Vec<u8>)>::new(),
            )))
            .app_data(Data::new(PipelineStore::new(Vec::new())))
            .service(render_fractal)
            .wrap(middleware::Logger::default())
    })
    .bind((ip, port))?
    .run()
    .await
}

#[actix_web::test]
async fn fractals_endpoint_test() {
    env_logger::Builder::new()
        .filter_module(grimoire::LOGGING_TARGET, log::LevelFilter::Debug)
        .filter_module("actix_web", log::LevelFilter::Info)
        .filter_module("actix_server", log::LevelFilter::Info)
        .filter_module("wgpu", log::LevelFilter::Off)
        .format_timestamp(None)
        .init();

    let mut app = actix_web::test::init_service(
        App::new()
            .app_data(Data::new(PipelineStore::new(Vec::new())))
            .app_data(Data::new(Mutex::new(
                Vec::<(RequestIdentifier, Vec<u8>)>::new(),
            )))
            .data_factory(|| async { generate_backend().await })
            .service(render_fractal)
            .wrap(middleware::Logger::default()),
    )
    .await;
    //I'm putting them all in one test, bc I had some issues with multiple tests that use the gpu

    //Mandelbrot
    let req = actix_web::test::TestRequest::with_uri("/fractals/Mandelbrot?colors=ffffff,11ffff,1100ff&position_x=-.1&position_y=1&zoom=10&debug=true&width=1024&height=1024&max_iterations=2000&num_colors=2000")
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);

    //Mandelbrot
    //Double request to test the cache
    let req = actix_web::test::TestRequest::with_uri("/fractals/Mandelbrot?colors=ffffff,11ffff,1100ff&position_x=-.1&position_y=1&zoom=10&debug=true&width=1024&height=1024&max_iterations=2000&num_colors=2000")
    .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);

    //Burning ship
    let req = actix_web::test::TestRequest::with_uri("/fractals/BurningShip?colors=ffffff,11ffff,1100ff&position_x=-.1&position_y=1&zoom=10&debug=true&width=1024&height=1024&max_iterations=2000&num_colors=2000")
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);

    //Tricorn
    let req = actix_web::test::TestRequest::with_uri("/fractals/Tricorn?colors=ffffff,11ffff,1100ff&position_x=-.1&position_y=1&zoom=10&debug=true&width=1024&height=1024&max_iterations=2000&num_colors=2000")
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);

    //Feather
    let req = actix_web::test::TestRequest::with_uri("/fractals/Feather?colors=ffffff,11ffff,1100ff&position_x=-.1&position_y=1&zoom=10&debug=true&width=1024&height=1024&max_iterations=2000&num_colors=2000")
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);

    //Eye
    let req = actix_web::test::TestRequest::with_uri("/fractals/Eye?colors=ffffff,11ffff,1100ff&position_x=-.1&position_y=1&zoom=10&debug=true&width=1024&height=1024&max_iterations=2000&num_colors=2000")
        .to_request();
    let resp = actix_web::test::call_service(&mut app, req).await;
    let status = resp.status();
    assert_eq!(status, actix_web::http::StatusCode::OK);
}
