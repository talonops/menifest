use std::{path::Path, thread::sleep, time::Duration};
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    let mut disks = Disks::new_with_refreshed_list();

    let gb = 1024 * 1024 * 1024;

    // prime the cpu so first reading isn't 0
    sys.refresh_cpu_usage();
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    loop {
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        disks.refresh(false);

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
            "CPU: {:.1}% | RAM: {} GB / {} GB | Disk: {} GB / {} GB",
            cpu,
            ram_used / gb,
            ram_total / gb,
            disk_used / gb,
            disk_total / gb
        );

        sleep(Duration::from_secs(5));
    }
}