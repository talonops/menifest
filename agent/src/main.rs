use std::{path::Path, thread::sleep, time::Duration};
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();

    let gb = 1024 * 1024 * 1024;
    let mb = 1024 * 1024;

    // prime the cpu so first reading isn't 0
    sys.refresh_cpu_usage();
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    loop {
        sleep(Duration::from_secs(5));

        sys.refresh_cpu_usage();
        sys.refresh_memory();
        disks.refresh(false);
        networks.refresh(false);

        let mut rx = 0u64;
        let mut tx = 0u64;
        for (name, data) in &networks {
            if name == "lo" {
                continue;
            } // Skip loopback
            rx += data.received();
            tx += data.transmitted();
        }
        let cpu = sys.global_cpu_usage();
        let ram_used = sys.used_memory();
        let ram_total = sys.total_memory();

        let root = disks
            .iter()
            .find(|d| d.mount_point() == Path::new("/"))
            .expect("no root disk found??");
        let disk_used = root.total_space() - root.available_space();
        let disk_total = root.total_space();

        println!(
            "CPU: {:.1}% | RAM: {} GB / {} GB | Disk: {} GB / {} GB | {}MB/s Received, {}MB/s Transmitted",
            cpu,
            ram_used / gb,
            ram_total / gb,
            disk_used / gb,
            disk_total / gb,
            rx / 5 / mb,
            tx / 5 / mb
        );
    }
}
