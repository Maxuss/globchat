use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type MessageId = i64;
pub type ChannelId = i64;
pub type UserId = Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub username: String,
    pub password: String,
    pub id: UserId,
    pub timestamp: u64,
    pub messages: Vec<MessageId>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageData {
    pub author: UserId,
    pub channel: ChannelId,
    pub id: MessageId,
    pub timestamp: u64,
    pub contents: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelData {
    pub name: String,
    pub id: ChannelId,
    pub timestamp: u64,
    pub creator: UserId,
    pub messages: Vec<MessageId>
}