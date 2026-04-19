use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
    pub cred: String,
}

pub async fn connect_server(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(body): Json<ConnectRequest>,
) -> Result<String, (StatusCode, String)> {
    ssh::connect(body)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok("connected".to_string())
}

pub async fn heartbeat(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(body): Json<shared::HeartbeatRequest>,
) {
    let mut hasher = Sha256::new();
    hasher.update(body.token.as_bytes());
    let hashed_token = hex::encode(hasher.finalize());

    let conn = db.lock().await;

    let exists = conn
        .query_row(
            "SELECT 1 FROM servers WHERE id = ? AND token_hash = ?",
            (&body.vps_id, &hashed_token),
            |_| Ok(true),
        )
        .unwrap_or(false);

    if exists {
        println!("exists")
    } else {
        println!("does not exisat")
    }
}
