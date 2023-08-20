pub mod auth;
pub  mod info;

use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use axum::{debug_handler, Router, ServiceExt};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use snowflake::SnowflakeIdGenerator;
use tracing::info;
use crate::db::Database;
use crate::routes::auth::{auth_login, auth_register, auth_status};
use crate::routes::info::{info_channel, info_messages, info_user};
use crate::state::{AppState, ConnectedClients, Snowflakes};

#[tracing::instrument(skip(db))]
pub async fn launch_api(db: Database) -> anyhow::Result<()> {
    let state = AppState {
        database: db,
        connected_clients: ConnectedClients(Arc::new(Mutex::new(Vec::with_capacity(8)))),
        snowflakes: Snowflakes(Arc::new(Mutex::new(SnowflakeIdGenerator::new(1, 1)))),
        jwt_secret: env::var("JWT_SECRET")?
    };

    let router = Router::new()
        .route("/auth/status", get(auth_status))
        .route("/auth/login", post(auth_login))
        .route("/auth/register", post(auth_register))
        .route("/info/user/:user_id", get(info_user))
        .route("/info/channel/:channel_id", get(info_channel))
        .route("/info/messages/:channel_id", get(info_messages))
        .with_state(state);

    axum::Server::bind(&SocketAddr::from_str("127.0.0.1:14670")?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

#[debug_handler]
async fn test_route() -> impl IntoResponse {
    info!("hello world!");
    return "!";
}