use std::sync::Arc;
use crate::repository::traits::UserRepository;
use crate::models::user::{NewUser, User};
use mongodb::bson::oid::ObjectId;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, user: NewUser) -> Result<User> {
        match self.repo.create_user(user.clone()).await {
            Ok(id) => {
                let user = User {
                    id: Some(id),
                    username: user.name,
                    email: 
                }
            }
            Err(e) => Err(e)
        }
    }

    pub async fn get_user(&self, id: &ObjectId) -> Result<User, String
}

