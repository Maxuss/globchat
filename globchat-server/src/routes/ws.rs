use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    headers,
    response::IntoResponse,
    TypedHeader,
};
use serde_json::json;
use tracing::info;

#[tracing::instrument(skip(ws, user_agent))]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket_wrap(socket, addr))
}

#[tracing::instrument(name = "handle_socket", skip(socket))]
async fn handle_socket_wrap(socket: WebSocket, who: SocketAddr) {
    if let Err(e) = handle_socket(socket, who).await {
        tracing::error!("Error when handling websocket for {who}: {e}");
    }
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) -> anyhow::Result<()> {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        info!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return Ok(());
    }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            info!("Got message: {msg:#?}");
            socket
                .send(Message::Text(serde_json::to_string(
                    &json! {{ "received": true }},
                )?))
                .await?;
        } else {
            println!("client {who} abruptly disconnected");
        }
    }
    Ok(())
}
