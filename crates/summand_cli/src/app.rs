use crate::commands::{self, Command};
use clap::Parser;
use summand_data::{adapters::dockdb_data_adapter::DockdbDataAdapter, data_adapter::DataAdapter};

pub struct Cli {
    pub data_adapter: DockdbDataAdapter,
    pub clap: CliClap,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliClap {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            clap: CliClap::parse(),
            data_adapter: DockdbDataAdapter::new(),
        }
    }

    pub fn init() -> Self {
        Self::new()
    }

    pub fn run(&self) -> std::process::ExitCode {
        if let Err(err) = commands::run_command(self.clap.command.clone()) {
            eprintln!("Error: {}", err);
            return std::process::ExitCode::FAILURE;
        }
        std::process::ExitCode::SUCCESS
    }
}
