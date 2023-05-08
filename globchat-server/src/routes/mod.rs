#![allow(clippy::let_with_type_underscore)]

mod ws;

use axum::{routing::get, Router};

use self::ws::ws_handler;

pub fn route<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/chat", get(ws_handler))
}
