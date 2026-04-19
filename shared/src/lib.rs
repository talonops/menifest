use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HeartbeatRequest {
    pub vps_id: String,
    pub token: String,
}