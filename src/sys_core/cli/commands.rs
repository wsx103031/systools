use std::collections::HashMap;

use crossterm::event::KeyCode;

use super::{controller::Controller, status::Terminating};

type Bind = Box<dyn FnMut(&mut Controller) -> std::io::Result<()>>;

pub fn command_base_set(set: &mut CommandSet, controller: &'static mut Controller) {
    set.commands.insert(
        KeyCode::Esc,
        Command::new(
            controller,
            "Quit".to_owned(),
            Box::new(Controller::terminate),
        ),
    );
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

    pub fn none() -> CommandSet {
        CommandSet {
            commands: HashMap::new(),
        }
    }
}

pub struct Command {
    controller: &'static mut Controller,
    description: String,
    bind: Bind,
}

impl Command {
    fn new(controller: &'static mut Controller, description: String, bind: Bind) -> Command {
        Command {
            controller,
            description,
            bind,
        }
    }
    pub fn execute(&mut self) -> std::io::Result<()> {
        let f = &mut self.bind;
        f(self.controller)?;
        Ok(())
    }
}
