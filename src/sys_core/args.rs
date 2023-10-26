use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ViewArgs {
    /// Displays different manitude of data,e.g., kb, mb, gb.
    #[arg(short, long,  action = clap::ArgAction::Count)]
    pub magnitude: u8,

    #[command(subcommand)]
    pub command: Option<Objective>,
}

#[derive(Subcommand, Debug)]
pub enum Objective {
    Component {},
    Cpu {},
    Disk {},
    Network {},
    Ram {},
    System {},
}
