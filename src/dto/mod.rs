use serde::Serialize;
use derive_more::From;

use crate::models::user::User;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_hex(),
            username: user.username,
            email: user.email,
        }
    }
}