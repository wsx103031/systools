use clap::Parser;
pub mod sys_core;
use sys_core::{args::*, sys_print::*};
use sysinfo::{System, SystemExt};

fn main() {
    //cli_test();
    //use functions from clap to construct custom commands we need.
    let args = ViewArgs::parse();

    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    match args.command {
        Some(Objective::System {}) => {
            println!("=> system:");
            print!("{}", print_system(&sys));
        }
        Some(Objective::Disk {}) => {
            println!("=> disks:");
            print!("{}", print_disks(&sys));
        }
        Some(Objective::Component {}) => {
            // Components temperature:
            println!("=> components:");
            print!("{}", print_components(&sys));
        }
        Some(Objective::Cpu {}) => {
            // Number of CPUs:
            print!("{}", print_cpu(&sys));
            print!("{}", print_processes(&sys));
        }
        Some(Objective::Network {}) => {
            // Network interfaces name, data received and data transmitted:
            println!("=> networks:");
            print!("{}", print_networks(&sys));
        }
        Some(Objective::Ram {}) => {
            print!("{}", print_ram(&sys));
        }
        None => {}
    }
}
