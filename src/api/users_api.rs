use actix_web::{HttpResponse, get, web, post};
use crate::{AppState, models::user::UserResponse, services::user_service};
use crate::models::user::{User, NewUser};
use validator::Validate;
use mongodb::bson::oid::ObjectId;
#[get("/")]
async fn list_users(state: web::Data<AppState>) -> HttpResponse {
    match state.user_service.get_users().await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
#[post("/")]
async fn add_user(state: web::Data<AppState>, body: web::Json<NewUser>) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(e)
    }

    let new_user = body.into_inner();
    match state.user_service.create_user(new_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}

#[get("/{user_id}")]
async fn add_user(state: web::Data<AppState>, web::Path<ObjectId>) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(e)
    }

    let new_user = body.into_inner();
    match state.user_service.create_user(new_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
    cfg.service(add_user);
}