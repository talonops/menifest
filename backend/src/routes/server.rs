use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use futures::Stream;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;
use ts_rs::TS;

use crate::{ssh, structs::ServerPublic};

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
    State(_db): State<Arc<Mutex<Connection>>>,
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
    if let Err(e) = conn.execute(
        "
            UPDATE servers SET
                last_heartbeat = ?,
                cpu = ?,
                ram_used = ?,
                ram_total = ?,
                disk_used = ?,
                disk_total = ?,
                net_rx = ?,
                net_tx = ?
            WHERE id = ?",
        (
            now,
            &body.cpu,
            &body.ram_used,
            &body.ram_total,
            &body.disk_used,
            &body.disk_total,
            &body.net_rx,
            &body.net_tx,
            &body.vps_id,
        ),
    ) {
        eprintln!("error updating server: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

pub async fn get_all(
    State(db): State<Arc<Mutex<Connection>>>,
) -> Result<Json<Vec<ServerPublic>>, StatusCode> {
    let conn = db.lock().await;

    let mut stmt = conn
        .prepare(
            "
    SELECT 
        id,
        name,
        last_heartbeat,
        created_at,
        cpu,
        ram_used,
        ram_total,
        disk_used,
        disk_total,
        net_rx,
        net_tx
    FROM servers",
        )
        .map_err(|e| {
            eprintln!("error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ServerPublic {
                id: row.get(0)?,
                name: row.get(1)?,
                last_heartbeat: row.get(2)?,
                created_at: row.get(3)?,
                cpu: row.get(4)?,
                ram_used: row.get(5)?,
                ram_total: row.get(6)?,
                disk_used: row.get(7)?,
                disk_total: row.get(8)?,
                net_rx: row.get(9)?,
                net_tx: row.get(10)?,
            })
        })
        .map_err(|e| {
            eprintln!("error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let servers: Vec<ServerPublic> = rows.filter_map(|r| r.ok()).collect();

    Ok(Json(servers))
}

pub async fn servers_stream(
    State(db): State<Arc<Mutex<Connection>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            yield Ok(Event::default().data(serde_json::json!({"message": "test data", "timestamp": std::time::SystemTime::now()}).to_string()));
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}
