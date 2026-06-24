use sysinfo::System;

fn main() {
    println!("telemetric script initialized");

    let relatory = system_info();
    telemetric_send(&relatory);
}

fn system_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let hostname = System::host_name();
    let system = System::name();
    let 
    
    let temperature = 
    let cpu_usage = sys.global_cpu_usage();

    let ram_total = sys.total_memory();
    let ram_used = sys.used_memory();
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    let relatory = "
        Telemetry script informations:
        Software:
        hostname = {hostname}

        Hardware:
        cpu usage {cpu_usage} 
        ram  {ram_used} / {ram_total} bytes
        swap {swap_used} / {swap_total} bytes
        "
}

fn telemetric_send(relatory: &str) {
    println!("{relatory}")
}
