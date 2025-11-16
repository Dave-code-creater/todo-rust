use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::dto::users::{UserResponse};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub retyped_password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,  
}


pub struct RegisterResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}
