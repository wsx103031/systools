#![recursion_limit = "10000"]
extern crate prettytable;
pub mod args;
pub mod cli;
pub mod sys_print;

use args::ViewArgs;
use clap::Parser;
use cli::controller::Controller;
use std::io::{self, Stdout};

fn main() -> std::io::Result<()> {
    //use functions from clap to construct custom commands we need.
    let args = ViewArgs::parse();
    let stdout: Stdout = io::stdout();
    let mut controller: Controller = Controller::new(stdout, args);
    controller.run()?;
    Ok(())
}
