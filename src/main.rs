#![recursion_limit = "10000"]
extern crate prettytable;
use clap::Parser;
pub mod sys_core;
use std::io::{self, Stdout};
use sys_core::{args::*, cli::controller::Controller};

fn main() -> std::io::Result<()> {
    //use functions from clap to construct custom commands we need.
    let args = ViewArgs::parse();
    let stdout: Stdout = io::stdout();
    let mut controller: Controller = Controller::new(stdout, args);
    controller.run()?;
    Ok(())
}
