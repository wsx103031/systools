use sysinfo::{NetworkExt, System, SystemExt};

pub fn print_system_status() {}

//static
pub fn print_disks(sys: &System) -> String {
    let mut res = String::from("");
    for disk in sys.disks() {
        res += &format!("{:?}\n", disk);
    }
    return res;
}

//static
pub fn print_components(sys: &System) -> String {
    let mut res = String::from("");
    for component in sys.components() {
        res += &format!("{:?}\n", component);
    }
    return res;
}

//dynamic
pub fn print_ram(sys: &System) -> String {
    let mut res = String::from("");
    // RAM and swap information:
    res += &format!(
        " total memory: {} bytes\n used memory : {} bytes\n total swap  : {} bytes\n used swap   : {} bytes\n",
        sys.total_memory(),
        sys.used_memory(),
        sys.total_swap(),
        sys.used_swap()
    );
    return res;
}

//dynamic
pub fn print_networks(sys: &System) -> String {
    let mut res = String::from("");
    for (interface_name, data) in sys.networks() {
        res += &format!(
            "{}: {}/{} B\n",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }
    return res;
}

//static
pub fn print_system(sys: &System) -> String {
    let mut res = String::from("");
    res += &format!(
        " System name:             {:?}\n System kernel version:   {:?}\n System OS version:       {:?}\n System host name:        {:?}\n",
        sys.name().unwrap(),
        sys.kernel_version().unwrap(),
        sys.os_version().unwrap(),
        sys.host_name().unwrap()
    );
    return res;
}

//static
pub fn print_cpu(sys: &System) -> String {
    return format!("NB CPUs: {}\n", sys.cpus().len());
}

//dynamic
pub fn print_processes(sys: &System) -> String {
    let mut res = String::from("");
    // for (pid, process) in sys.processes() {
    //     res += &format!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
    res += &format!("{}", sys.processes().len());
    return res;
}

pub fn print_division(n: usize) -> String {
    return "-".repeat(n);
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
