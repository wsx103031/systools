use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyEvent, KeyEventKind},
    execute, queue, style,
    terminal::{self, ClearType},
};

use sysinfo::{System, SystemExt};

use crate::sys_core::{
    args::{Objective, ViewArgs},
    sys_print::*,
};

use super::{commands::*, status::*};

pub struct Controller {
    writer: Box<dyn io::Write>,
    status: Status,
    args: ViewArgs,
    system: Box<System>,
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

    fn prepare(&mut self) -> &mut Self {
        self.system.refresh_all();
        self.status = Status::Ready;
        self
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        if !self.runnable() {
            self.prepare();
        }
        execute!(self.writer, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        self.status = Status::Running;
        let mut commands = command_base_set();
        while Status::Running == self.status {
            self.update(&mut commands)?;
        }
        execute!(self.writer, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn receive_command(&mut self, commands: &mut CommandSet) -> std::io::Result<()> {
        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            if let Some(command) = commands.get(code) {
                command.execute(self)?;
            }
        }
        Ok(())
    }

    fn update(&mut self, commands: &mut CommandSet) -> std::io::Result<()> {
        queue!(
            self.writer,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
        )?;
        self.refresh_screen()?;
        self.writer.flush()?;
        self.receive_command(commands)?;
        sleep(Duration::new(2, 0));
        Ok(())
    }
    pub fn refresh_screen(&mut self) -> std::io::Result<()> {
        match self.args.command {
            Some(Objective::System {}) => {
                self.system.refresh_system();
                println!("=> system:");
                print!("{}", print_system(&self.system));
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
            Some(Objective::Process { limit, interval: _ }) => {
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
        Ok(())
    }

    pub fn terminate(&mut self) -> std::io::Result<()> {
        self.status = Status::Terminating;
        Ok(())
    }
}
