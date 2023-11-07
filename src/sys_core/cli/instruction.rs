use std::{collections::HashMap, io::Result, thread};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use futures::{channel::mpsc, SinkExt};
use prettytable::{row, Table};

use super::controller::Controller;

type Bind = Box<dyn FnMut(&mut Controller) -> std::io::Result<()>>;

pub fn command_base_set() -> InstructionSet {
    let mut set = InstructionSet {
        instructions: HashMap::new(),
    };
    set.instructions.insert(
        KeyCode::Esc,
        Instruction::new(
            "Esc".to_owned(),
            "Quit".to_owned(),
            Box::new(Controller::terminate),
        ),
    );
    set.instructions.insert(
        KeyCode::Backspace,
        Instruction::new(
            "Backspace".to_owned(),
            "Update".to_owned(),
            Box::new(Controller::refresh_screen),
        ),
    );
    set
}

pub struct InstructionSet {
    instructions: HashMap<KeyCode, Instruction>,
}

impl InstructionSet {
    fn _add(mut self, key: KeyCode, com: Instruction) {
        self.instructions.insert(key, com);
    }

    pub fn get(&mut self, code: KeyCode) -> Option<&mut Instruction> {
        self.instructions.get_mut(&code)
    }

    pub fn hint(&mut self) -> Table {
        let mut table = Table::new();
        for (_, com) in &self.instructions {
            table.add_row(row![com.name, com.description]);
        }
        table
    }

    pub fn execute(&mut self, code: KeyCode, controller: &mut Controller) -> Result<()> {
        if let Some(command) = self.instructions.get_mut(&code) {
            command.execute(controller)?;
        };
        Ok(())
    }
}

pub struct Instruction {
    name: String,
    description: String,
    bind: Bind,
}

impl Instruction {
    fn new(name: String, description: String, bind: Bind) -> Instruction {
        Instruction {
            name,
            description,
            bind,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn execute(&mut self, controller: &mut Controller) -> std::io::Result<()> {
        let f = &mut self.bind;
        f(controller)?;
        Ok(())
    }
}

fn _rec_with_send(controller: &mut Controller, instructions: &mut InstructionSet) -> Result<()> {
    let (mut tx, mut rx) = mpsc::channel(1000);

    let handle = thread::spawn(move || {
        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            let _ = tx.send(code);
        }
    });

    while !handle.is_finished() {}
    let code = rx.try_next().unwrap().unwrap();

    if let Some(ins) = instructions.get(code) {
        ins.execute(controller)?;
    }

    handle.join().unwrap();
    _rec_with_send(controller, instructions)?;
    Ok(())
}
