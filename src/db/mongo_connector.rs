use mongodb::{Client, Database, options::ClientOptions};
use anyhow::Result;

#[derive(Clone)]
pub struct MongoConnector {
    client: Client,
    db: Database,
}

impl MongoConnector {
    pub async fn new(uri: &str) -> Result<Self> {
        let opts = ClientOptions::parse(uri).await?;
        let client = Client::with_options(opts.clone())?;
        let db_name = opts
            .default_database
            .clone()
            .expect("Database name must be included in the Mongo URI");

        let db = client.database(&db_name);

        Ok(Self { client, db })
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
}