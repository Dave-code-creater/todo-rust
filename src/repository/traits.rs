use async_trait::async_trait;
use anyhow::Result;
use mongodb::bson::oid::ObjectId;

use crate::models::user::{NewUser,User};
use crate::models::task::{NewTask, Task, UpdateTask};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<ObjectId>;
    async fn get_user(&self, id: &ObjectId) -> Result<Option<User>>;
    // Implement a methods to find email and username (These are unique)
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_tasks(&self, new_task: NewTask, user_id: ObjectId) -> Result<ObjectId>;
    async fn list_tasks_by_user(&self, user_id: ObjectId) -> Result<Vec<Task>>;
    async fn update_task(&self, task_id: ObjectId, tasks: UpdateTask) -> Result<Option<Task>>;
}