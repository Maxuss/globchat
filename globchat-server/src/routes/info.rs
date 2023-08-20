use axum::extract::{Path, Query, State};
use axum::Json;
use axum_macros::debug_handler;
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::Deserialize;
use crate::err::GlobError;
use crate::extract::Authenticated;
use crate::model::{ChannelId, MessageData, UserId};
use crate::response::{ChannelResponse, GlobResponse, MessageResponse, UserResponse};
use crate::state::AppState;

#[debug_handler]
pub async fn info_user(
    _: Authenticated,
    State(AppState { database, .. }): State<AppState>,
    Path(user_id): Path<UserId>
) -> GlobResponse<UserResponse> {
    let user = database.users.find_one(doc! { "id": user_id }, None).await?;
    if let Some(user) = user {
        Ok(Json(UserResponse {
            username: user.username,
            user_id: user.id,
            creation_time: user.timestamp
        }))
    } else {
        Err(GlobError::UserNotFound)
    }
}

#[debug_handler]
pub async fn info_channel(
    _: Authenticated,
    State(AppState { database, .. }): State<AppState>,
    Path(channel_id): Path<ChannelId>
) -> GlobResponse<ChannelResponse> {
    let channel = database.channels.find_one(doc! { "id": channel_id }, None).await?;
    if let Some(channel) = channel {
        Ok(Json(ChannelResponse {
            name: channel.name,
            creation_time: channel.timestamp,
            creator: channel.creator,
            channel_id: channel.id
        }))
    } else {
        Err(GlobError::ChannelNotFound)
    }
}

#[derive(Deserialize)]
pub struct MessageQuery {
    from: i64,
    to: Option<i64>
}

#[debug_handler]
pub async fn info_messages(
    _: Authenticated,
    State(AppState { database, .. }): State<AppState>,
    Path(channel_id): Path<ChannelId>,
    Query(MessageQuery { from, to }): Query<MessageQuery>
) -> GlobResponse<Vec<MessageResponse>> {
    let messages = database.messages.find(doc! { "timestamp": { "$gte": from, "$lte": to }, "channel": channel_id }, None).await?;
    let messages: Vec<MessageData> = messages
        .try_collect()
        .await
        .map_err(|_| GlobError::MessageNotFound)?;

    Ok(Json(messages.into_iter().map(|each| MessageResponse { contents: each.contents, author: each.author, timestamp: each.timestamp, message_id: each.id }).collect()))
}