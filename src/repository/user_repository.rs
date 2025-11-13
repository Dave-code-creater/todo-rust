use anyhow::Result;
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::{doc, oid::ObjectId}, Collection, Database};
use crate::{
    models::user::{User, UserResponse},
    repository::traits::UserRepository,
};


pub struct MongoUserRepo {
    col: Collection<User>,
}

impl MongoUserRepo {
    pub fn new(db: &Database) -> Self {
        let col = db.collection::<User>("users");
        Self { col }
    }
}


#[async_trait]
impl UserRepository for MongoUserRepo {
    async fn create_user(&self, user: &User) -> Result<ObjectId> {
        let res = self.col.insert_one(user).await?;
        let id = res
            .inserted_id
            .as_object_id()
            .ok_or_else(|| anyhow::anyhow!("inserted_id is not an ObjectId"))?;
        Ok(id)
    }

    async fn get_all_users(&self) -> Result<Vec<User>> {
        let cursor = self.col.find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    async fn get_user_by_id(&self, id: &ObjectId) -> Result<Option<User>> {
        let user = self.col.find_one(doc! { "_id": id }).await?;
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = self.col.find_one(doc! {"email": email}).await?;
        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = self.col.find_one(doc! {"username": username}).await?;
        Ok(user)
    }
}