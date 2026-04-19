use serde::Deserialize;
use std::{path::Path, time::Duration};
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};

#[derive(Deserialize)]
struct Config {
    vps_id: String,
    token: String,
    backend_url: String,
}

#[tokio::main]
async fn main() {
    let text = std::fs::read_to_string("config.toml").expect("config.toml not found");
    let config: Config = toml::from_str(&text).expect("config.toml is malformed");

    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();

    let client = reqwest::Client::new();

    sys.refresh_cpu_usage();
    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;

    let url = format!("{}/heartbeat", config.backend_url);
    let interval_secs = 5;

    loop {
        tokio::time::sleep(Duration::from_secs(interval_secs)).await;

        sys.refresh_cpu_usage();
        sys.refresh_memory();
        disks.refresh(false);
        networks.refresh(false);

        let mut rx = 0u64;
        let mut tx = 0u64;
        for (name, data) in &networks {
            if name == "lo" {
                continue;
            }
            rx += data.received();
            tx += data.transmitted();
        }

        let root = disks
            .iter()
            .find(|d| d.mount_point() == Path::new("/"))
            .expect("no root disk found");

        let body = shared::HeartbeatRequest {
            vps_id: config.vps_id.clone(),
            token: config.token.clone(),
            cpu: sys.global_cpu_usage(),
            ram_used: sys.used_memory() as i64,
            ram_total: sys.total_memory() as i64,
            disk_used: (root.total_space() - root.available_space()) as i64,
            disk_total: root.total_space() as i64,
            net_rx: (rx / interval_secs) as i64,
            net_tx: (tx / interval_secs) as i64,
        };

        match client.post(&url).json(&body).send().await {
            Ok(res) => println!("✅ {}", res.status()),
            Err(e) => eprintln!("❌ {}", e),
        }
    }
}
