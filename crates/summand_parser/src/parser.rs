use summand_app::summand::Summand;

use crate::adapters::{file_adapter::FileAdapter, json_adapter::JsonAdapter};

pub struct SummandParser {}

impl SummandParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, text: &str) -> Summand {
        let adapter = JsonAdapter {};
        return adapter.parse(text);
    }
}
