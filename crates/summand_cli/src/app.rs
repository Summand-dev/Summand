use crate::commands::{self, Command};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn init() -> Self {
        Self::new()
    }

    pub fn run(&self) -> std::process::ExitCode {
        if let Err(err) = commands::run_command(self.command.clone()) {
            eprintln!("Error: {}", err);
            return std::process::ExitCode::FAILURE;
        }
        std::process::ExitCode::SUCCESS
    }
}
