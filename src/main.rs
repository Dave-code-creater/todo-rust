mod api;
mod db;
mod models;
mod repository;
mod services;
mod dto;
use env_logger::Env;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::{env, sync::Arc};
use actix_web::middleware::Logger;
use crate::db::mongo_connector::MongoConnector;
use crate::repository::user_repository::MongoUserRepo;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub mongo: MongoConnector,
    pub user_service: UserService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri = env::var("MONGOURI").expect("MONGOURI not set");
    env_logger::init_from_env(
            Env::default().default_filter_or("info")
        );    let mongo = MongoConnector::new(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let user_repo = Arc::new(MongoUserRepo::new(mongo.db()));

    let user_service = UserService::new(user_repo);

    let state = web::Data::new(AppState {
        mongo,
        user_service,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(state.clone())
            .configure(api::init)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}