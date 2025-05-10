use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummandVariable {
    pub name: String,
    pub description: String,
    pub value: Option<String>,
    pub is_env: bool,
    pub can_override: bool,
}

impl SummandVariable {
    pub fn new(
        name: &str,
        description: Option<&str>,
        value: Option<&str>,
        is_env: Option<bool>,
        can_override: Option<bool>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.unwrap_or("").to_string(),
            value: match value {
                Some(v) => Option::from(v.to_string()),
                None => None,
            },
            is_env: is_env.unwrap_or(false).to_owned(),
            can_override: can_override.unwrap_or(false).to_owned(),
        }
    }

    pub fn reset(&mut self) {
        self.value = Option::from("".to_string());
    }
}

impl fmt::Display for SummandVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Summand Variable(name: {}, value: {}, is_env: {}, can_override: {})",
            self.name,
            self.value.as_ref().unwrap(),
            self.is_env,
            self.can_override
        )
    }
}
