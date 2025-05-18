pub mod version;
pub mod add;
pub mod list;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Version,
    Add(add::AddArgs),
    List(list::ListArgs),
}

pub async fn run_command(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Version => version::handle().await,
        Command::Add(args) => add::handle(args).await,
        Command::List(args) => list::handle(args).await,
    }
}