use std::{collections::HashMap, io::Result};

use crossterm::event::KeyCode;
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
        KeyCode::Enter,
        Instruction::new(
            "Enter".to_owned(),
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
