use async_trait::async_trait;
use anyhow::Result;
use mongodb::bson::oid::ObjectId;

use crate::models::user::{NewUser,User};
use crate::models::task::{NewTask, Task, UpdateTask};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, new_user: NewUser) -> Result<ObjectId>;
    async fn get_user(&self, id: &ObjectId) -> Result<Option<User>>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_tasks(&self, new_task: NewTask, user_id: ObjectId) -> Result<ObjectId>;
    async fn list_tasks_by_user(&self, user_id: ObjectId) -> Result<Vec<Task>>;
    async fn update_task(&self, task_id: ObjectId, tasks: UpdateTask) -> Result<Option<Task>>;
}