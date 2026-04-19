use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HeartbeatRequest {
    pub vps_id: String,
    pub token: String,
    pub cpu: f32,
    pub ram_used: u64,
    pub ram_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub net_rx: u64,
    pub net_tx: u64
}