use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use ts_rs::TS;

use crate::ssh;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export)]
pub struct ConnectRequest {
    pub ip: String,
    pub port: u16,
    pub username: String,
    //pub key: bool, // key or password
    pub cred: String
}

pub async fn connect_server(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(body): Json<ConnectRequest>,
) -> Result<String, (StatusCode, String)> {
    ssh::connect(body).await.map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok("connected".to_string())
}