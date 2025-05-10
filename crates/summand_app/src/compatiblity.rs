use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SummandCompatiblityType {
    Platform,
    Capablity,
}

impl fmt::Display for SummandCompatiblityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SummandCompatiblityType::Capablity => "CapablityFilter",
                SummandCompatiblityType::Platform => "PlatformFilter",
            }
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummandCompatiblity {
    pub compatiblity_type: SummandCompatiblityType,
    pub allow: Vec<String>,
    pub disallow: Vec<String>,
}

impl SummandCompatiblity {
    pub fn new(
        compatiblity_type: SummandCompatiblityType,
        allow: Option<Vec<String>>,
        disallow: Option<Vec<String>>,
    ) -> Self {
        Self {
            compatiblity_type: compatiblity_type,
            allow: allow.or(Option::from(Vec::new())).unwrap(),
            disallow: disallow.or(Option::from(Vec::new())).unwrap(),
        }
    }
}

impl fmt::Display for SummandCompatiblity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Summand Compatiblity(type: {}, allow: {:?}, disallow: {:?})",
            self.compatiblity_type, self.allow, self.disallow
        )
    }
}
