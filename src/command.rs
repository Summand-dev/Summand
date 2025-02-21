use std::process::Output;

use crate::argument::CommandArgument;

#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub program: String,
    pub arguments: Vec<CommandArgument>,
    pub output: Option<Output>,
}

impl Command {
    pub fn new(
        name: &str,
        description: Option<&str>,
        program: &str,
        arguments: Option<Vec<CommandArgument>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.unwrap_or("").to_string(),
            program: program.to_string(),
            arguments: arguments.unwrap_or(Vec::new()),
            output: None,
        }
    }
}
