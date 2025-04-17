mod cli;
mod commands;

use cli::CliRunner;

fn main() -> std::process::ExitCode {
    CliRunner::init().run()
}
