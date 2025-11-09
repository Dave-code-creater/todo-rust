use anyhow::Result;
use async_trait::async_trait;
use mongodb::{bson::{doc, oid::ObjectId}, Collection};
use crate::{
    db::mongo_connector::MongoConnector,
    models::user::{NewUser, User},
    repository::traits::UserRepository,
};


pub struct MongoUserRepo {
    col: Collection<User>,
}

impl MongoUserRepo {
    pub fn new(conn: &MongoConnector) -> Self {
        let col = conn.db().collection::<User>("users");
        Self {col}
    }
}


#[async_trait]
impl UserRepository for MongoUserRepo {
    async fn create_user(&self, new_user: NewUser) -> Result<ObjectId> {
        let user = User {
            id: None, 
            username: new_user.username,
            email: new_user.email,
            password: new_user.password,
            task_ids: Vec::new(),
        };

        let res = self.col.insert_one(&user).await?;

        let id = res
            .inserted_id
            .as_object_id()
            .ok_or_else(|| anyhow::anyhow!("Inserted_id not an ObjectID"))?;

        Ok(id)
    }

    async fn get_user(&self, id: &ObjectId) -> Result<Option<User>> {
        let user = self.col.find_one(doc! { "_id": id }).await?;
        Ok(user)
    }
}