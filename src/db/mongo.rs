use mongodb::{Client, Database, option::ClientOptions};
use anyhow::Result;

pub struct MongoConnector {
    client: Client,
    db: Database,
}

impl MongoConnector {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self> {
        let opts = ClientOptions::parse(uri).await?;
        let client = Client::with_options(opts)?;
        let db = client.database(db_name);

        Ok(Self {client, db})
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
}