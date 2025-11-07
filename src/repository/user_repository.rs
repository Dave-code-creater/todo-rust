use anyhow::Result;
use mongodb::{bson::{doc, oid::ObjectId}, Collection};
use crate::{db::mongo_connector::MongoConnector, models::user::User};
use crate::repository::traits::UserRepository;
use async_trait::async_trait;

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
    async fn create_user(&self, mut user: User) -> Result<ObjectId>{
        let res = self.col.insert_one(&user, None).await?;
        let id = res.inserted_id().as_object_id().ok_or_else(
            || anyhow::anyhow!("Inserted_id not an ObjectID")
        )?;
        user.id = Some(id);
        Ok(id)
    }
    async fn get_user(&self, id: ObjectId) -> Result<Option<User>> {
        let user = self.col.find_one(doc! {"_id": id}, None).await?;
        Ok(user)
    }
}