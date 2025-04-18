use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SummandConfig {
    pub name: String,
    pub description: Option<String>,
    pub compatiblity: Option<Vec<Compatiblity>>,
    pub capability: Option<Vec<Capability>>,
    pub strategy: Option<Strategy>,
    pub commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Capability {
    pub name: String,
    pub version: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Compatiblity {
    pub r#type: String,
    pub aloow: Vec<String>,
    pub disallow: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Strategy {
    pub run_strategy: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub program: String,
    pub args: Vec<CommandArgument>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommandArgument {
    pub name: String,
    pub value: Option<String>,
}
