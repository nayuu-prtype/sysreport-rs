use indoc::indoc;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use sysinfo::{Components, Disks, Networks, System};

const BYTES_IN_GIGABYTES: f64 = 1024.0 * 1024.0 * 1024.0;
const BYTES_IN_MEGABYTES: f64 = 1024.0 * 1024.0;

fn main() {
    println!("Sysreport initialized");

    let report = system_info();
    telemetric_send(&report);
}

fn system_info() -> String {
    // refresh system
    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let components = Components::new_with_refreshed_list();
    let networks = Networks::new_with_refreshed_list();
    let load_avg = System::load_average();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    sys.refresh_memory();

    // system
    let hostname = System::host_name().unwrap_or_else(|| "failed".to_string());
    let system = System::name().unwrap_or_else(|| "failed".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "failed".to_string());

    // uptime
    let mut up: f64 = (System::uptime() as f64) / 60.0;
    let uptime = if up < 60.0 {
        format!("{up:.1}min")
    } else if up < 1440.0 {
        up /= 60.0;
        format!("{up:.1}h")
    } else if up < 43200.0 {
        up = up / 60.0 / 24.0;
        format!("{up:.1}d")
    } else {
        up = up / 60.0 / 24.0 / 30.0;
        format!("{up:.1}mo")
    };

    // cpu usage
    let cpu_usage = sys.global_cpu_usage();
    let cpu_number = sys.cpus().len();
    let cpu_avg_one = load_avg.one;
    let cpu_avg_five = load_avg.five;
    let cpu_avg_fifteen = load_avg.fifteen;

    // ram usage
    let ram_total: f64 = (sys.total_memory() as f64) / BYTES_IN_GIGABYTES;
    let ram_used: f64 = (sys.used_memory() as f64) / BYTES_IN_GIGABYTES;
    let swap_total: f64 = (sys.total_swap() as f64) / BYTES_IN_GIGABYTES;
    let swap_used: f64 = (sys.used_swap() as f64) / BYTES_IN_GIGABYTES;

    // temperature
    let mut temperature_info = String::new();
    for component in &components {
        let temp_name = component.label();
        let temp_temp = component.temperature().unwrap_or(0.0);

        if temp_temp == 0.0 {
            continue;
        }

        let temp_format = format!("\n{:.1} °C  {}", temp_temp, temp_name);
        temperature_info.push_str(&temp_format);
    }

    // network
    let mut network_info = String::new();
    for (interface_name, data) in &networks {
        let interface = format!("network: {}", interface_name);

        let dataup: f64 = (data.total_transmitted() as f64) / BYTES_IN_MEGABYTES;
        let datado: f64 = (data.total_received() as f64) / BYTES_IN_MEGABYTES;

        if dataup < 0.01 && datado < 0.01 {
            continue;
        };

        let data_up = if dataup < 1024.0 {
            format!("data: {:.2}MB (up)", dataup)
        } else {
            format!("data: {:.2}GB (up)", dataup / 1024.0)
        };

        let data_down = if datado < 1024.0 {
            format!("data: {:.2}MB (down)", datado)
        } else {
            format!("data: {:.2}GB (down)", datado / 1024.0)
        };

        let data_format = format!("\n{}\n{}\n{}\n", interface, data_down, data_up);
        network_info.push_str(&data_format);
    }

    // disk
    let mut disks_info = String::new();
    for disk in &disks {
        let disk_name = disk.name();
        let disk_mount = disk.mount_point();
        let disk_format = disk.file_system();
        let disk_type = disk.kind();

        let disk_space_total: f64 = (disk.total_space() as f64) / BYTES_IN_GIGABYTES;
        let disk_space_available: f64 = (disk.available_space() as f64) / BYTES_IN_GIGABYTES;
        let disk_space_used: f64 = disk_space_total - disk_space_available;

        let disk_space = format!(
            "{disk_space_used:.2?}/{disk_space_total:.2?}GB, available: {disk_space_available:.2?}GB"
        );
        let disk_format = format!(
            "\nname: {disk_name:?}\ndisk: {disk_space}\nformat: {disk_format:?}\nmount: {disk_mount:?}\ntype: {disk_type:?}\n"
        );
        disks_info.push_str(&disk_format);
    }

    let report = format!(
        indoc! {"```
            Sysreport:

            ┌ Software ┐
            │hostname  │   {}
            │          │
            │system    │   {}
            │kernel    │   {}
            │uptime    │   {}
            └──────────┘

            ┌ Hardware ──────┐
            │cpu usage       │   {:.2}% CPU usage
            │                │
            │cpu avg one     │   {:.1}/{} CPUs
            │cpu avg five    │   {:.1}/{} CPUs
            │cpu avg fifteen │   {:.1}/{} CPUs
            │                │
            │ram usage       │   {:.1}/{:.1}GB RAM
            │swap usage      │   {:.1}/{:.1}GB RAM
            └────────────────┘

            Networks
            {}
            Disks
            {}
            Temperature
            {}```"
        },
        hostname,
        system,
        kernel_version,
        uptime,
        cpu_usage,
        cpu_avg_one,
        cpu_number,
        cpu_avg_five,
        cpu_number,
        cpu_avg_fifteen,
        cpu_number,
        ram_used,
        ram_total,
        swap_used,
        swap_total,
        network_info,
        disks_info,
        temperature_info
    );

    report
}

fn telemetric_send(report: &str) {
    let client = Client::new();

    dotenvy::dotenv().ok();
    let webhook_url = env::var("WEBHOOK").expect("no url found");

    client
        .post(webhook_url)
        .json(&json!({"content": report.trim() }))
        .send()
        .expect("error send mensage");

    print!("finalizing sysreport");
}
