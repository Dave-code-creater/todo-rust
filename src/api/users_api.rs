use actix_web::{get,  web, HttpResponse};
use crate::services::user_service;

#[get("/")]
async fn list_users() -> HttpResponse {
    user_servi
    HttpResponse::Ok().json("Hello world")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
}