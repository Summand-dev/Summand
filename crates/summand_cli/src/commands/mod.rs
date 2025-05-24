pub mod version;
pub mod add;
pub mod list;
pub mod import;
pub mod export;
pub mod run;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Version,
    Add(add::AddArgs),
    List(list::ListArgs),
    Import(import::ImportArgs),
    Export(export::ExportArgs),
    Run(run::RunArgs),
}

pub async fn run_command(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Version => version::handle().await,
        Command::Add(args) => add::handle(args).await,
        Command::List(args) => list::handle(args).await,
        Command::Import(args) => import::handle(args).await,
        Command::Export(args) => export::handle(args).await,
        Command::Run(args) => run::handle(args).await,
    }
}