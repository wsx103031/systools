use prettytable::*;
use sysinfo::{DiskExt, NetworkExt, ProcessExt, System, SystemExt};

//static
pub fn print_disks(sys: &System) -> String {
    let mut table = Table::new();

    table.add_row(row![
        "Name",
        "System",
        "Type",
        "removable",
        "mounted on",
        "Unuse",
        "Total"
    ]);
    for disk in sys.disks() {
        let name = disk.name().to_str().unwrap();
        let system: String = String::from_utf8(disk.file_system().to_owned()).unwrap();
        let kind: String = format!("{:?}", disk.kind());
        let removeable = disk.is_removable();
        let mounted_on = format!("{:?}", disk.mount_point());
        let unused = disk.available_space();
        let total = disk.total_space();

        table.add_row(row![
            name, system, kind, removeable, mounted_on, unused, total
        ]);
    }
    return table.to_string();
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
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    // RAM and swap information:
    let table = table![
        ["Attributes", "Values"],
        ["Total memory", total_mem],
        ["Used memory", used_mem],
        ["Total swap", total_swap],
        ["Used swap", used_swap]
    ];
    return table.to_string();
}

//dynamic
pub fn print_networks(sys: &System) -> String {
    let mut table = Table::new();
    table.add_row(row!["Name", "received", "transmitted"]);
    for (interface_name, data) in sys.networks() {
        table.add_row(row![interface_name, data.received(), data.transmitted()]);
    }
    return table.to_string();
}

//static
pub fn print_system(sys: &System) -> String {
    let name = sys.name().unwrap_or_default();
    let kernel_ver = sys.kernel_version().unwrap_or_default();
    let os_ver = sys.os_version().unwrap_or_default();
    let host_name = sys.host_name().unwrap_or_default();
    let mut table = Table::new();
    table.add_row(row![bFg->"Attributes",bFg->"Values"]);
    table.add_row(row!["System name", &name]);
    table.add_row(row!["System kernel version", &kernel_ver]);
    table.add_row(row!["System OS version", &os_ver]);
    table.add_row(row!["System Host name", &host_name]);
    return table.to_string();
}

//static
pub fn print_cpu(sys: &System) -> String {
    return format!("NB CPUs: {}\n", sys.cpus().len());
}

//dynamic
pub fn print_processes(sys: &mut System, limit: u8) -> String {
    let mut table = Table::new();
    table.add_row(row!["pid", "Name", "written", "read"]);
    let mut i = 0_u8;
    for (pid, process) in sys.processes() {
        if i == limit {
            break;
        }
        table.add_row(row![
            pid,
            process.name(),
            process.disk_usage().written_bytes,
            process.disk_usage().read_bytes
        ]);
        i += 1;
    }
    return table.to_string();
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
