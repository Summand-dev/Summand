use crate::commands::{self, Command};
use clap::Parser;
use summand_data::{adapters::sqlx_data_adapter::SQLxDataAdapter, data_adapter::DataAdapter};

pub struct Cli {
    // pub data_adapter: SQLxDataAdapter,
    pub clap: CliClap,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliClap {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub async fn new() -> Self {
        Cli {
            clap: CliClap::parse(),
            // data_adapter: SQLxDataAdapter::new().await,
        }
    }

    pub async fn init() -> Self {
        Self::new().await
    }

    pub async fn run(&self) -> std::process::ExitCode {
        if let Err(err) = commands::run_command(self.clap.command.clone()).await {
            eprintln!("Error: {}", err);
            return std::process::ExitCode::FAILURE;
        }
        std::process::ExitCode::SUCCESS
    }
}
