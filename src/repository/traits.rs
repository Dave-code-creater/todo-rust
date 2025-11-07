use async_trai::async_trait;
use anyhow::Result;
use mongodb::bson::oid::ObjectId;
use crate::models::{User, Task};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, mut user: User) -> Result<ObjectId>;
    async fn get_user(&self, id: ObjectId) -> Result<Option<User>>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_task(&self, mut task: Task) -> Result<ObjectId>;
    async fn list_tasks_by_user(&self, user_id: ObjectId) -> Result<Vec<Task>>;
    async fn update_task(&self, mut task: Task, task_id: ObjectId) -> Result<Option<Task>>;
}