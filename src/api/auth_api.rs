use actix_web::{
    get,
    web::Path,
    web::Json,
    web
};
use serde::{
    Serialize,
    Deserialize
};

use crate::dto::auth::RegisterRequest;
use validator::Validate;

#[get("/auth/login")]
pub async fn login(state: web::Data<AppState>, body: web::Json<RegisterRequest>) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(e);
    }


}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(login);
}