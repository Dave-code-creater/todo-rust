use actix_web::{HttpResponse, get, web, post};
use crate::{AppState, models::user::UserResponse, services::user_service};
use crate::models::user::{User, NewUser};

#[get("/")]
async fn list_users(state: web::Data<AppState>) -> HttpResponse {
    match state.user_service.get_users().await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
#[post("/")]
async fn add_user(state: web::Data<AppState>, req: web::Json<NewUser>) -> HttpResponse {
    let new_user = req.into_inner();
    match state.user_service.create_user(new_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
    cfg.service(add_user);
}