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

    ws.on_upgrade(move |socket| handle_socket_wrap(socket, addr))
}

#[tracing::instrument(name = "handle_socket", skip(socket))]
async fn handle_socket_wrap(mut socket: WebSocket, who: SocketAddr) {
    if let Err(e) = handle_socket(&mut socket, who).await {
        tracing::error!("Error when handling websocket for {who}: {e}");
        socket
            .send(Message::Text(
                serde_json::to_string(&json! {{ "error": e.to_string() }}).unwrap(),
            ))
            .await
            .ok();
    }
}

async fn handle_socket(socket: &mut WebSocket, who: SocketAddr) -> anyhow::Result<()> {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            info!("Got message: {msg:#?}");

            match msg {
                Message::Text(txt) => {
                    let res: serde_json::Value = serde_json::from_str(&txt)?;
                    socket
                        .send(Message::Text(serde_json::to_string(
                            &json! {{ "received": true, "echo": res }},
                        )?))
                        .await?;
                }
                Message::Binary(_) => {
                    socket
                        .send(Message::Text(serde_json::to_string(
                            &json! {{ "error": "binary messages not supported" }},
                        )?))
                        .await?;
                    socket.send(Message::Close(None)).await?;
                    return Ok(());
                }
                Message::Close(_) => break,
                _ => {
                    // ignoring other packets
                    continue;
                }
            }
        } else {
            info!("client {who} abruptly disconnected");
        }
    }

    info!("client {who} disconnected");

    Ok(())
}
