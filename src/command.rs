use crate::argument::Argument;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub command: String,
    pub arguments: Vec<Argument>,
}

impl Command {
    pub fn new(
        name: &str,
        description: Option<&str>,
        command: &str,
        arguments: Option<Vec<Argument>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.unwrap_or("").to_string(),
            command: command.to_string(),
            arguments: arguments.unwrap_or(Vec::new()),
        }
    }
}
