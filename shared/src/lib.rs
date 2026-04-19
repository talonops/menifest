use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HeartbeatRequest {
    pub vps_id: String,
    pub token: String,
    pub cpu: f32,
    pub ram_used: i64,
    pub ram_total: i64,
    pub disk_used: i64,
    pub disk_total: i64,
    pub net_rx: i64,
    pub net_tx: i64,
}