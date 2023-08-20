use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use crate::db::Database;
use axum::extract::FromRef;
use snowflake::SnowflakeIdGenerator;
use uuid::Uuid;

pub type JwtSecret = String;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub database: Database,
    pub connected_clients: ConnectedClients,
    pub snowflakes: SnowflakeIdGenerator,
    pub jwt_secret: JwtSecret,
}

#[derive(Debug, Clone)]
pub struct ConnectedClients(pub Arc<Mutex<Vec<ClientData>>>);

impl ConnectedClients {
    pub fn insert(&self, id: Uuid) {
        self.0.lock().unwrap().push(ClientData { uid: id });
    }

    pub fn pop(&self, id: Uuid) {
        let mut this = self.0.lock().unwrap();
        *this = this.clone().into_iter().filter(|it| it.uid != id).collect();
    }
}

#[derive(Debug, Clone)]
pub struct ClientData {
    pub uid: Uuid
}