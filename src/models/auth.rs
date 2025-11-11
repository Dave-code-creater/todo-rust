use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(messages = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "password must be att least 8 characters"))]
    pub password: String, 
}