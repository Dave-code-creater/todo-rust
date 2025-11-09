use std::sync::Arc;
use crate::repository::traits::UserRepository;
use crate::models::user::{NewUser, User, UserResponse};
use mongodb::bson::oid::ObjectId;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User> {

        let id = self.repo.create_user(new_user.clone()).await?;

        match self.repo.create_user(new_user.clone()).await {
            Ok(id) => {
                let user = UserResponse {
                    id: Some(id),
                    username: new_user.name,
                    email: new_user.email,
                    task_id: Vec::new(),
                };
                Oke(user)
            }
            Err(e) => Err(e)
        }
    }

    pub async fn get_user(&self, id: &ObjectId) -> Result<User, String
}

