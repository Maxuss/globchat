use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use crate::db::Database;
use axum::extract::FromRef;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub database: Database,
    pub connected_clients: ConnectedClients
}

#[derive(Debug, Clone)]
pub struct ConnectedClients(pub Arc<Mutex<Vec<ClientData>>>);

#[derive(Debug, Clone)]
pub struct ClientData {

}