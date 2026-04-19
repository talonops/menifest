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
) -> StatusCode {

    match ssh::connect(body).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            // TODO: log properly
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn heartbeat(
    State(db): State<Arc<Mutex<Connection>>>,
    Json(body): Json<shared::HeartbeatRequest>,
) -> StatusCode {
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

    if !exists {
        return StatusCode::UNAUTHORIZED;
    }

    let now = chrono::Utc::now().timestamp();
    if conn
        .execute(
            "UPDATE servers SET last_heartbeat = ? WHERE id = ?",
            (now, &body.vps_id),
        )
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
