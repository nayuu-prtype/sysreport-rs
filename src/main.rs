use sysinfo::{System, Components};

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
    let mut temperature_cpu = String::new();
    for component in &components {
        if let Some(temperature) = component.temperature()  {
            temperature_cpu = temperature.to_string();
        } else {
            temperature_cpu = "unknow".to_string();
        }
    }

    let relatory = format!("
        Telemetry script:

        ┌ Software ─┐
        │hostname   │   {hostname}
        │system            {system}
        │kernel         {kernel_version}

        Hardware:
        cpu usage         {cpu_usage:.2} 
        ram usage         {ram_used:.2}/{ram_total:.2}GB
        swap usage        {swap_used:.2}/{swap_total:.2}GB
        temperature       {temperature_cpu} °C
        ");

        return relatory;
}

fn telemetric_send(relatory: &str) {
    println!("{relatory}")
}
