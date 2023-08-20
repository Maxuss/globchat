pub mod auth;

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
use crate::state::{AppState, ConnectedClients};

#[tracing::instrument(skip(db))]
pub async fn launch_api(db: Database) -> anyhow::Result<()> {
    let state = AppState {
        database: db,
        connected_clients: ConnectedClients(Arc::new(Mutex::new(Vec::with_capacity(8)))),
        snowflakes: SnowflakeIdGenerator::new(1, 1),
        jwt_secret: env::var("JWT_SECRET")?
    };

    let router = Router::new()
        .route("/auth/status", get(auth_status))
        .route("/auth/login", post(auth_login))
        .route("/auth/register", post(auth_register))
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