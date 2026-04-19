use serde::{Deserialize, Serialize};
use ts_rs::TS;

/* 
pub struct Server {
    pub id: String,
    pub name: String,
    pub token_hash: String,
    pub last_heartbeat: i64,
    pub created_at: i64,

    // stats
    pub cpu: f32,
    pub ram_used: u64,
    pub ram_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub net_rx: u64,
    pub net_tx: u64,
}*/

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export)]
pub struct ServerPublic {
    pub id: String,
    pub name: String,
    pub last_heartbeat: i64,
    pub created_at: i64,
    pub cpu: Option<f32>,
    pub ram_used: Option<i64>,
    pub ram_total: Option<i64>,
    pub disk_used: Option<i64>,
    pub disk_total: Option<i64>,
    pub net_rx: Option<i64>,
    pub net_tx: Option<i64>,
}