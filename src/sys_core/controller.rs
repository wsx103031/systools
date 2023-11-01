use std::io::{self, Write};

use crossterm::{execute, queue, style, terminal};
use prettytable::table;
use sysinfo::{System, SystemExt};

use crate::sys_core::sys_print::*;

use super::args::{Objective, ViewArgs};

pub struct Controller {
    writer: Box<dyn io::Write>,
    status: Status,
    args: ViewArgs,
    system: Box<System>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Status {
    Inactive,
    Ready,
    Running,
    Stopping,
    Terminating,
}

/// 先創建必要物件
pub trait Begin {
    fn prepare(&mut self) -> &mut Self;
    fn run(&mut self) -> std::io::Result<&Status>;
}
/// 開新執行緒監控鍵盤輸入、更新console資訊
trait Running {
    fn receive_keycode(&mut self) -> std::io::Result<&'static Status>;
    fn update(&mut self);
    fn refresh_screen(&mut self);
}

impl Controller {
    pub fn new<W>(stdout: W, args: ViewArgs) -> Controller
    where
        W: io::Write + 'static,
    {
        Controller {
            writer: Box::new(stdout),
            status: Status::Inactive,
            args,
            system: Box::new(System::new_all()),
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    fn runnable(&self) -> bool {
        self.status == Status::Ready || self.status == Status::Stopping
    }

    fn hint(&self) -> String {
        let base = table![
            ["Esc", "p", "n", "u"],
            ["Quit", "Previous", "Next", "Update"]
        ];
        base.to_string()
    }
}

impl Begin for Controller {
    fn prepare(&mut self) -> &mut Self {
        self.system.refresh_all();
        self.status = Status::Ready;
        self
    }

    fn run(&mut self) -> std::io::Result<&Status> {
        if !self.runnable() {
            self.prepare();
        }

        execute!(self.writer, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        queue!(self.writer, style::Print("123"))?;
        self.writer.flush()?;
        // self.status = Status::Running;
        // while self.runnable() {
        //     self.update();
        //     self.stdout.flush()?;
        // }
        execute!(self.writer, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(&self.status)
    }
}

impl Running for Controller {
    fn receive_keycode(&mut self) -> std::io::Result<&'static Status> {
        Ok(&Status::Running)
    }

    fn update(&mut self) {}
    fn refresh_screen(&mut self) {
        match self.args.command {
            Some(Objective::System {}) => {
                self.system.refresh_system();

                println!("=> system:");
                print!("{}{}", print_system(&self.system), self.hint());
            }
            Some(Objective::Disk {}) => {
                self.system.refresh_disks();
                println!("=> disks:");
                print!("{}", print_disks(&self.system));
            }
            Some(Objective::Component {}) => {
                self.system.refresh_components();
                // Components temperature:
                println!("=> components:");
                print!("{}", print_components(&self.system));
            }
            Some(Objective::Cpu { limit, interval: _ }) => {
                self.system.refresh_processes();
                // Number of CPUs:
                print!("{}", print_cpu(&self.system));
                print!("{}", print_processes(&mut self.system, limit));
            }
            Some(Objective::Network {}) => {
                self.system.refresh_networks();
                // Network interfaces name, data received and data transmitted:
                println!("=> networks:");
                print!("{}", print_networks(&self.system));
            }
            Some(Objective::Ram {}) => {
                self.system.refresh_memory();
                print!("{}", print_ram(&self.system));
            }
            None => {}
        }
        self.status = Status::Terminating;
    }
}
