use sysinfo::{System, Components};
use reqwest::blocking::Client;
use serde_json::json;
use std::env;

const BYTES_IN_GIGABYTES: f64 = 1024.0 * 1024.0 * 1024.0;

fn main() {
    println!("telemetric script initialized");

    let relatory = system_info();
    telemetric_send(&relatory);
}

fn system_info() -> String {

    // refresh system
    let mut sys = System::new_all();
    let components = Components::new_with_refreshed_list();
    sys.refresh_all();

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
    let mut temperature_all = String::new();
    for component in &components {
        let name = component.label();
        let temp = component.temperature().unwrap_or(0.0);

        if temp == 0.0 {
            continue;
        }

        let temp_info = format!("{:<18} {:.1} °C\n", name, temp);
        temperature_all.push_str(&temp_info);
    }

    let relatory = format!("
Telemetry script:

┌ Software ┐
│hostname  │   {hostname}
│          │
│system    │   {system}
│kernel    │   {kernel_version}
└──────────┘

┌ Hardware ──┐
│cpu usage   │   {cpu_usage:.2} 
│ram usage   │   {ram_used:.2}/{ram_total:.2}GB
│swap usage  │   {swap_used:.2}/{swap_total:.2}GB
└────────────┘

temperature

{temperature_all}");

        return relatory;
}

fn telemetric_send(relatory: &str) {
    let client = Client::new();

    dotenvy::dotenv().expect("could not load .env");
    let webhook_url = env::var("WEBHOOK")
        .expect("no url found");

    client.post(webhook_url)
        .json(&json!({"content": relatory.trim() }))
        .send()
        .expect("error send mensage");
}
