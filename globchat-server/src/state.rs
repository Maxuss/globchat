use std::sync::{Arc, Mutex};
use crate::db::Database;
use axum::extract::FromRef;
use snowflake::SnowflakeIdGenerator;
use crate::model::UserId;

pub type JwtSecret = String;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub database: Database,
    pub connected_clients: ConnectedClients,
    pub snowflakes: Snowflakes,
    pub jwt_secret: JwtSecret,
}

#[derive(Debug, Clone)]
pub struct Snowflakes(pub Arc<Mutex<SnowflakeIdGenerator>>);

impl Snowflakes {
    pub fn generate(&self) -> i64 {
        let mut lock = self.0.lock().unwrap();
        return lock.generate()
    }
}

#[derive(Debug, Clone)]
pub struct ConnectedClients(pub Arc<Mutex<Vec<ClientData>>>);

impl ConnectedClients {
    pub fn insert(&self, id: UserId) {
        self.0.lock().unwrap().push(ClientData { uid: id });
    }

    pub fn pop(&self, id: UserId) {
        let mut this = self.0.lock().unwrap();
        *this = this.clone().into_iter().filter(|it| it.uid != id).collect();
    }
}

#[derive(Debug, Clone)]
pub struct ClientData {
    pub uid: UserId
}