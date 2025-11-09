// External crate imports
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection
};

use crate::{
    db::mongo_connector::MongoConnector,
    models::task::{NewTask, Task, UpdateTask},
    repository::traits::TaskRepository,
};

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
    async fn create_tasks(&self, mut task: NewTask) -> Result<ObjectId> {
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


    async fn update_task(
        &self,
        task_id: ObjectId,
        update_task: UpdateTask,
    ) -> Result<Option<Task>> {
        let mut update_doc = to_document(&update_task)?;
        update_doc.remove("_id");
        update_doc.insert("edit_date", mongodb::bson::DateTime::now());

        // build options
        let opts = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let updated = self
            .col
            .find_one_and_update(
                doc! { "_id": task_id },
                doc! { "$set": update_doc },
            )
            .with_options(opts)
            .await?;

        if let Some(doc) = updated {
            let task: Task = mongodb::bson::from_document(doc)?;
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }
}
