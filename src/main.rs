mod api;
mod models;
mod repository;

use actix_web::{App,HttpResponse,  HttpServer, Responder, get, middleware::Logger, web::{self, Data}};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
    .bind(("localhost", 8080))?
    .run()
    .await
}
