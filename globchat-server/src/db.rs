use mongodb::bson::Document;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use tracing::info;
use crate::model::{ChannelData, MessageData, UserData};

#[tracing::instrument(skip(pwd))]
pub async fn mongo_connect(pwd: &str) -> anyhow::Result<Database> {
    let uri = format!("mongodb://globchat:{pwd}@localhost:27017/globchat?retryWrites=true");

    let mut opts = ClientOptions::parse(uri).await?;
    opts.app_name = Some(String::from("globchat-server"));

    let client = Client::with_options(opts)?;
    let db = client.database("globchat");

    info!("Connected to MongoDB successfully!");

    Database::init(db).await
}

#[derive(Debug, Clone)]
pub struct Database {
    pub inner: mongodb::Database,
    pub messages: Collection<MessageData>,
    pub users: Collection<UserData>,
    pub channels: Collection<ChannelData>
}

impl Database {
    pub async fn init(db: mongodb::Database) -> anyhow::Result<Self> {
        Ok(Self {
            messages: db.collection("messages"),
            users: db.collection("users"),
            channels: db.collection("channels"),
            inner: db,
        })
    }
}