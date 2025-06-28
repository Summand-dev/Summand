pub mod add;
pub mod export;
pub mod import;
pub mod list;
pub mod run;
pub mod version;

use clap::Subcommand;
use summand_app::app::application::Application;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Version,
    Add(add::AddArgs),
    List(list::ListArgs),
    Import(import::ImportArgs),
    Export(export::ExportArgs),
    Run(run::RunArgs),
}

pub async fn run_command(command: Command, app: &Application) -> anyhow::Result<()> {
    match command {
        Command::Version => version::handle(app).await,
        Command::Add(args) => add::handle(args).await,
        Command::List(args) => list::handle(args).await,
        Command::Import(args) => import::handle(args).await,
        Command::Export(args) => export::handle(args).await,
        Command::Run(args) => run::handle(args).await,
    }
}
