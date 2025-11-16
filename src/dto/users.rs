use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

