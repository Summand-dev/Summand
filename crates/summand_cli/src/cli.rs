use crate::commands::{self, Cli};
use clap::Parser;

pub struct CliRunner {
    cli: Cli,
}

impl CliRunner {
    pub fn new() -> Self {
        let cli = Cli::parse();
        Self { cli }
    }

    pub fn init() -> Self {
        Self::new()
    }

    pub fn run(&self) -> std::process::ExitCode {
        if let Err(err) = commands::run_command(self.cli.command.clone()) {
            eprintln!("Error: {}", err);
            return std::process::ExitCode::FAILURE;
        }
        std::process::ExitCode::SUCCESS
    }
}
