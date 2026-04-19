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
    pub cpu: f32,
    pub ram_used: i64,
    pub ram_total: i64,
    pub disk_used: i64,
    pub disk_total: i64,
    pub net_rx: i64,
    pub net_tx: i64,
}