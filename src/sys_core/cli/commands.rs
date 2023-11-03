use std::{collections::HashMap, error::Error};

use crossterm::event::KeyCode;
use prettytable::{row, Table};

use super::controller::Controller;

type Bind = Box<dyn FnMut(&mut Controller) -> std::io::Result<()>>;

pub fn command_base_set() -> CommandSet {
    let mut set = CommandSet {
        commands: HashMap::new(),
    };
    set.commands.insert(
        KeyCode::Esc,
        Command::new(
            "Esc".to_owned(),
            "Quit".to_owned(),
            Box::new(Controller::terminate),
        ),
    );
    set.commands.insert(
        KeyCode::Backspace,
        Command::new(
            "Backspace".to_owned(),
            "Update".to_owned(),
            Box::new(Controller::refresh_screen),
        ),
    );
    set
}

pub struct CommandSet {
    commands: HashMap<KeyCode, Command>,
}

impl CommandSet {
    fn add(mut self, key: KeyCode, com: Command) {
        self.commands.insert(key, com);
    }

    pub fn get(&mut self, code: KeyCode) -> Option<&mut Command> {
        self.commands.get_mut(&code)
    }

    pub fn hint(&mut self) -> Table {
        let mut table = Table::new();
        for (_, com) in &self.commands {
            table.add_row(row![com.name, com.description]);
        }
        table
    }

    pub fn execute(
        &mut self,
        code: KeyCode,
        controller: &mut Controller,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(command) = self.commands.get_mut(&code) {
            command.execute(controller)?;
        };
        Ok(())
    }
}

pub struct Command {
    name: String,
    description: String,
    bind: Bind,
}

impl Command {
    fn new(name: String, description: String, bind: Bind) -> Command {
        Command {
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
