use crate::models::user::NewUser;
use crate::AppState;
use actix_web::{HttpResponse, get, post, web};
use mongodb::bson::oid::ObjectId;
use validator::Validate;

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
        return HttpResponse::BadRequest().json(e);
    }

    let new_user = body.into_inner();
    match state.user_service.create_user(new_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[get("/{user_id}")]
async fn get_user(state: web::Data<AppState>, user_id: web::Path<String>) -> HttpResponse {
    let id_str = user_id.into_inner();
    println!("Raw user id from path: {}", id_str);

    let id = match ObjectId::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest()
                .body("Invalid user id (must be a 24-char hex ObjectId)");
        }
    };

    match state.user_service.get_user(&id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) if e.to_string() == "User not found" => {
            HttpResponse::NotFound().body("User not found")
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
    cfg.service(add_user);
    cfg.service(get_user);
}
