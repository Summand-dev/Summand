#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub command: String,
    pub arguments: Vec<String>,
}

impl Command {
    pub fn new(
        name: String,
        description: Option<String>,
        command: String,
        arguments: Option<Vec<String>>,
    ) -> Self {
        Self {
            name: String::from(name),
            description: description.unwrap_or(String::from("")),
            command: String::from(command),
            arguments: arguments.unwrap_or(vec![]),
        }
    }
}
