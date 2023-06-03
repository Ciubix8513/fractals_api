use actix_web::{HttpResponse, Responder};

#[actix_web::get("/coloring")]
async fn coloring_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./pages/coloring.html"))
}

#[actix_web::get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./pages/index.html"))
}
