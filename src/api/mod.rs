use actix_web::web;

mod users_api;
mod task_api;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users").configure(users_api::init))
        .service(web::scope("/tasks").configure(task_api::init));
}