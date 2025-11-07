use std::sync::Arc;
use crate::repository::traits::UserRepository;
use crate::models::user::User;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn register(&self, user: User) -> Result<User, String> {
        match self.repo.create_user(user.clone()).await {
            Ok(id) => {
                user.id = Some(id);
                Ok(user)
            }
            Err(e) => Err(e)
        }
    }
}

