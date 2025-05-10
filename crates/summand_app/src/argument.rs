use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandArgument {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl CommandArgument {
    pub fn new(name: Option<&str>, value: Option<&str>) -> Self {
        Self {
            name: match name {
                Some(n) => Some(n.to_string()),
                None => None,
            },
            value: match value {
                Some(v) => Some(v.to_string()),
                None => None,
            },
        }
    }
}
