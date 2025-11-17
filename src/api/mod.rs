use actix_web::web;

mod user_api;
mod task_api;
mod auth_api;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users").configure(user_api::init))
        .service(web::scope("/tasks").configure(task_api::init))
        .service(web::scope("/auth").configure(auth_api::init));
}

