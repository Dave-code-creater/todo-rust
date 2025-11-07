use async_trait::async_trait;
use anyhow::Result;
use mongodb::bson::oid::ObjectId;

use crate::models::user::{NewUser,User};
use crate::models::task::{NewTask, Task};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, new_user: NewUser) -> Result<User>;
    async fn get_user(&self, id: &ObjectId) -> Result<Option<User>>;
}

// #[async_trait]
// pub trait TaskRepository: Send + Sync {
//     async fn 
// }