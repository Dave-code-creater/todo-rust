use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{
    Serialize,
    Deserialize
};
use derive_more::{Display};

use crate::repository::




#[post("/auth/signup")]
pub async fn sign_up(body: Json<SignUpIndentifier>) -> Json<String>{
    
}