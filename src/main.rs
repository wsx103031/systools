#![recursion_limit = "10000"]
extern crate prettytable;
use clap::Parser;
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
};
pub mod sys_core;
use std::io::{self, stdout, Stdout};
use sys_core::{args::*, cli::controller::Controller};

async fn _receive_keycode() -> std::io::Result<KeyCode> {
    execute!(stdout(), EnableMouseCapture)?;

    loop {
        // Blocking read
        let event: Event = read()?;

        match event {
            Event::Key(c) => match c.code {
                KeyCode::Enter => {}
                _ => {
                    execute!(stdout(), DisableMouseCapture)?;
                    return Ok(c.code);
                }
            },
            ref _default => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    //use functions from clap to construct custom commands we need.
    let args = ViewArgs::parse();
    let stdout: Stdout = io::stdout();
    let mut controller: Controller = Controller::new(stdout, args);
    controller.run()?;
    Ok(())
    // match block_on(receive_keycode()) {
    //     Ok(code) => match code {
    //         KeyCode::Char('c') => println!("123"),
    //         KeyCode::Char('q') => {}
    //         _ => {}
    //     },
    //     Err(v) => return Err(v),
    // }
}
