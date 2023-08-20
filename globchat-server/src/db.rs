use mongodb::bson::Document;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use tracing::info;

#[tracing::instrument(skip(pwd))]
pub async fn mongo_connect(pwd: &str) -> anyhow::Result<Database> {
    let uri = format!("mongodb://globchat:{pwd}@localhost:27017/globchat?retryWrites=true");

    let mut opts = ClientOptions::parse(uri).await?;
    opts.app_name = Some(String::from("globchat-server"));

    let client = Client::with_options(opts)?;
    let db = client.database("globchat");

    info!("Connected to MongoDB successfully!");

    Ok(Database {
        inner: db
    })
}

#[derive(Debug, Clone)]
pub struct Database {
    pub inner: mongodb::Database
}