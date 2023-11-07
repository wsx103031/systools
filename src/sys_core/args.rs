use std::path::PathBuf;

use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ViewArgs {
    /// Displays different manitude of data,e.g., kb, mb, gb.
    #[arg(short, long,  action = clap::ArgAction::Count)]
    pub magnitude: u8,
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Objective,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Objective {
    Component {},
    Process {
        #[arg(default_value = "3")]
        interval: u64,
        #[arg(default_value = "10")]
        limit: u8,
    },
    Disk {},
    Network {},
    Ram {},
    System {},
}

impl Objective {
    pub fn require_dynamic_update(&self) -> bool {
        match self {
            Objective::Process { .. } => true,

            Objective::Network {} => true,
            Objective::Ram {} => true,
            _other => false,
        }
    }
}

impl std::fmt::Display for Objective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Objective::Component {} => write!(f, "Components"),
            Objective::Process {
                limit: _,
                interval: _,
            } => write!(f, "Processes"),
            Objective::Disk {} => write!(f, "Disks"),
            Objective::Network {} => write!(f, "Network"),
            Objective::Ram {} => write!(f, "Ram"),
            Objective::System {} => write!(f, "System"),
        }
    }
}
