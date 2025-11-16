use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.expect("REASON").to_hex(),
            username: user.username,
            email: user.email,
        }
    }
}