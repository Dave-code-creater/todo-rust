mod api;
mod db;
mod models;
mod repository;
mod services;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;

use crate::db::mongo_connector::MongoConnector;
use crate::services::user_service::UserService;
use crate::services::{user_service};

#[derive(Clone)]
pub struct AppState {
    pub mongo: MongoConnector,
    pub user_service: UserService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo_uri = env::var("MONGOURI").expect("MONGOURI not set");
    let connector = MongoConnector::new(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");

    // Make it shareable for all routes
    let connector_data = web::Data::new(connector);
    HttpServer::new(move || {
        App::new()
            .app_data(connector_data.clone())
            .configure(api::init)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}