pub mod version;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Version,
}

pub fn run_command(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Version => version::handle(),
    }
}
