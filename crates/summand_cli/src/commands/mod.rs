pub mod version;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Version,
}

pub fn run_command(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Version => version::handle(),
    }
}
