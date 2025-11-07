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


#[derive(Deserialize, Serialize)]
pub struct SignUpIndentifier {
    name: String,
    email: String,
    password: String,
}

#[post("/auth/signup")]
pub async fn sign_up(body: Json<SignUpIndentifier>) -> Json<String>{
    
}