use std::sync::Arc;
use crate::repository::traits::UserRepository;
use crate::models::user::{NewUser, User};
use crate::dto::user::UserResponse;
use bcrypt::{hash, DEFAULT_COST};
use anyhow::Result;
use mongodb::bson::oid::ObjectId;
#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<UserResponse> {

        // User exists ? 
        if self.repo.find_by_email(&new_user.email).await?.is_some() {
            anyhow::bail!("User with this email already exists");
        }

        if self.repo.find_by_username(&new_user.username).await?.is_some() {
            anyhow::bail!("User with this username already exist");
        }


        // Hash password
        let hashed = hash(&new_user.password, DEFAULT_COST)?;  
        
        let user = User {
            id: None,
            username: new_user.username.clone(),
            email: new_user.email.clone(),
            password: hashed,
            task_ids: Vec::new(),
        };

        // Insert to DB
        let id = self.repo.create_user(&user).await?;

        // Return safe user response (no password)
        Ok(UserResponse {
            id: id.to_hex(),
            username: new_user.username,
            email: new_user.email,
        })
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        let users: Vec<User> = self.repo.get_all_users().await?;
        Ok(users)
    }

    pub async fn get_user(&self, id: &ObjectId) -> Result<UserResponse> {
        let user = self.repo
        .get_user_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        Ok(user.into())
    }

    
}
