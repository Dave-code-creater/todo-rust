use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub user_id: ObjectId,
    pub description: Option<String>, 
    pub is_completed: Option<bool>,
    pub due_date: Option<DateTime>,
    pub create_date: Option<DateTime>,
    pub edit_date: Option<DateTime>, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String, 
    pub is_completed: bool,
    pub due_date: DateTime,
    pub create_date: DateTime,
    pub edit_date: DateTime,
    pub user_id: ObjectId,
}