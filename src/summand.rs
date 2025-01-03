use crate::command::Command;

#[derive(Debug)]
pub struct Summand {
    pub name: String,
    pub description: String,
    pub commands: Vec<Command>,
}

impl Summand {
    pub fn new(name: &str, description: Option<&str>, commands: Option<Vec<Command>>) -> Self {
        Self {
            name: name.to_string(),
            description: description.unwrap_or("").to_string(),
            commands: commands.unwrap_or(Vec::new()),
        }
    }
}
