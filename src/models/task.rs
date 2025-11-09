use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub due_date: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub due_date: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub due_date: DateTime,
    pub create_date: DateTime,
    pub edit_date: DateTime,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub due_date: Option<String>,
    pub create_date: String,
    pub edit_date: String,
}