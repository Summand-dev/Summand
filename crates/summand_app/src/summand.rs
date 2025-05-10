use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    capability::SummandCapability,
    command::Command,
    compatiblity::SummandCompatiblity,
    strategy::{RunBreakStrategy, SummandStrategy},
    variable::SummandVariable,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summand {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub commands: Vec<Command>,
    pub variables: Vec<SummandVariable>,
    pub compatiblities: Vec<SummandCompatiblity>,
    pub capabilities: Vec<SummandCapability>,
    pub strategy: SummandStrategy,
}

impl Summand {
    pub fn new(name: &str, description: Option<&str>, commands: Option<Vec<Command>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.unwrap_or("").to_string(),
            commands: commands.unwrap_or(Vec::new()),
            variables: Vec::new(),
            compatiblities: Vec::new(),
            capabilities: Vec::new(),
            strategy: SummandStrategy::new(crate::strategy::RunStrategies::Break(
                RunBreakStrategy::new(),
            )),
        }
    }
}
