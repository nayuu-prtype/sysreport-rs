use sysinfo::{Components, Disks, Networks, System};
use reqwest::blocking::Client;
use serde_json::json;
use indoc::indoc;
use std::env;

const BYTES_IN_GIGABYTES: f64 = 1024.0 * 1024.0 * 1024.0;
const BYTES_IN_MEGABYTES: f64 = 1024.0 * 1024.0;

fn main() {
    println!("telemetric script initialized");

    let report = system_info();
    telemetric_send(&report);
}

fn system_info() -> String {

    // refresh system
    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let components = Components::new_with_refreshed_list();
    let networks = Networks::new_with_refreshed_list();
    sys.refresh_all();

    // system
    let hostname = System::host_name().unwrap_or("failed".to_string());
    let system = System::name().unwrap_or("failed".to_string());
    let kernel_version = System::kernel_version().unwrap_or("failed".to_string());

    // cpu usage
    let cpu_usage = sys.global_cpu_usage();

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
        let interface = interface_name;
        let data_up: f64 = (data.total_received() as f64) / BYTES_IN_MEGABYTES;
        let data_down = (data.total_transmitted() as f64) / BYTES_IN_MEGABYTES;

        if data_up == 0.0 || data_down == 0.0 {
            continue;
        };

        let data_format = format!("\nnetwork: {interface}\ndata down: {data_down:.2}MB\ndata up: {data_up:.2}MB\n");
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

        let disk_space = format!("{disk_space_used:.2?}/{disk_space_total:.2?}GB, avaible: {disk_space_available:.2?}GB");
        let disk_format = format!("\nname: {disk_name:?}\ndisk: {disk_space}\nformat: {disk_format:?}\nmount: {disk_mount:?}\ntype: {disk_type:?}\n");
        disks_info.push_str(&disk_format);
    }

    let report = format!(
        indoc! {"```
            Telemetry script:

            ┌ Software ┐
            │hostname  │   {hostname}
            │          │
            │system    │   {system}
            │kernel    │   {kernel}
            └──────────┘

            ┌ Hardware ──┐
            │cpu usage   │   {cpu:.2}%
            │ram usage   │   {ram_u:.1}/{ram_t:.1}GB
            │swap usage  │   {swap_u:.1}/{swap_t:.1}GB
            └────────────┘

            Networks
            {network}
            Disks
            {disks}
            Temperature
            {temperature}```"
        },

        hostname = hostname,
        system = system,
        kernel = kernel_version,
        cpu = cpu_usage,
        ram_u = ram_used,
        ram_t = ram_total,
        swap_u = swap_used,
        swap_t = swap_total,
        temperature = temperature_info,
        network = network_info,
        disks = disks_info
    );

    return report;
}

fn telemetric_send(report: &str) {
    let client = Client::new();

    dotenvy::dotenv().expect("could not load .env");
    let webhook_url = env::var("WEBHOOK")
        .expect("no url found");

    client.post(webhook_url)
        .json(&json!({"content": report.trim() }))
        .send()
        .expect("error send mensage");
}
