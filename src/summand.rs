use crate::command;

pub struct Summand {
    pub name: String,
    pub description: String,
    pub commands: Vec<command::Command>,
}

impl Summand {
    pub fn new(
        name: String,
        description: Option<String>,
        commands: Option<Vec<command::Command>>,
    ) -> Self {
        Self {
            name: name,
            description: description.unwrap_or(String::from("")),
            commands: commands.unwrap_or(vec![]),
        }
    }
}
