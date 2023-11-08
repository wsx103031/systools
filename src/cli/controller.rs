use std::{
    io::{self, Write},
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyEvent, KeyEventKind},
    execute, queue,
    style::{self, Print},
    terminal::{self, ClearType},
};

use sysinfo::{System, SystemExt};

use crate::{
    args::{Objective, ViewArgs},
    sys_print::*,
};

use super::instruction::*;

pub struct Controller {
    /// Should remove in here. And then add transmitter and receiver to handle what relate with stdout.
    writer: Box<dyn io::Write>,
    status: Status,
    /// Should remove in here. Turn it into pure parameter in the beginning.
    args: ViewArgs,
    system: System,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Status {
    Inactive,
    Running,
    Terminating,
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
            system: System::new_all(),
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        execute!(self.writer, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        self.status = Status::Running;
        let mut commands = command_base_set();

        self.update(&mut commands)?;

        execute!(self.writer, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn update(&mut self, instructions: &mut InstructionSet) -> std::io::Result<()> {
        queue!(self.writer, style::ResetColor, cursor::Hide)?;
        self.refresh_screen()?;

        let (tx, rx) = mpsc::channel();
        let status = Arc::new(Mutex::new(self.status.clone()));
        let status_in = status.clone();
        let handle = thread::spawn(move || {
            while Status::Running == *status_in.lock().unwrap() {
                if let Ok(Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                })) = event::read()
                {
                    tx.send(code).unwrap();
                }
            }
        });

        while Status::Running == self.status {
            while let Ok(code) = rx.try_recv() {
                instructions.execute(code, self)?;
            }
        }
        *status.lock().unwrap() = self.status.clone();
        handle.join().unwrap();
        Ok(())
    }

    pub fn refresh_screen(&mut self) -> std::io::Result<()> {
        let res = match self.args.command {
            Objective::System {} => print_system(&mut self.system),
            Objective::Disk {} => print_disks(&mut self.system),
            Objective::Component {} => print_components(&mut self.system),
            Objective::Process { limit, interval: _ } => print_processes(&mut self.system, limit),
            Objective::Network {} => print_networks(&mut self.system),
            Objective::Ram {} => print_ram(&mut self.system),
        };
        queue!(
            self.writer,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(1, 1),
            Print(format!("=> {}:\n{}", &mut self.args.command, res))
        )?;

        self.writer.flush()?;
        Ok(())
    }

    pub fn terminate(&mut self) -> std::io::Result<()> {
        self.status = Status::Terminating;
        Ok(())
    }
}
