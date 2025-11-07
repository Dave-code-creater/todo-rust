use anyhow::Result;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
    Cursor,
};
use futures::stream::StreamExt;
use crate::{db::mongo_connector::MongoConnector, models::task::Task};
use crate::repository::traits::TaskRepository;
use async_trait::async_trait;

pub struct MongoTaskRepo {
    col: Collection<Task>
}

impl MongoTaskRepo{
    pub fn new(conn: &MongoConnector) -> Self {
        let col = conn.db().collection::<Task>("tasks");
        Self {col}
    }
}

#[async_trait]
impl TaskRepository for MongoTaskRepo {
    async fn create_tasks(&self, mut task: Task) -> Result<ObjectId> {
        let res = self.col.insert_one(&task).await?;
        let id = res.inserted_id.as_object_id().ok_or_else(|| anyhow::anyhow!("Inserted_id not an ObjectId"))?;
        task.id = Some(id);
        Ok(id)
    }

    async fn list_tasks_by_user(&self, user_id: ObjectId) -> Result<Vec<Task>> {
        let mut cursor = self.col.find(doc! {"user_id": user_id}).await?;
        let mut tasks = Vec::new();

        while let Some(result) = cursor.next().await {
            let task = result?;
            tasks.push(task)
        }
        Ok(tasks)
    }

     async fn update_task(&self, task_id: ObjectId, task: Task) -> Result<Option<Task>> {
        let mut update_doc = to_document(&task)?;
        // Never attempt to set `_id`
        update_doc.remove("_id");

        let opts = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After) 
            .build();

        let updated = self
            .col
            .find_one_and_update(
                doc! { "_id": task_id },
                doc! { "$set": update_doc }
            )
            .await?;

        Ok(updated)
    }
}
