use core::fmt;
use std::{collections::HashMap, str::FromStr};

use regex::Regex;

use crate::summand::Summand;

#[derive(Debug)]
pub struct SummandWorkspace {
    pub env_variables: HashMap<String, String>,
    pub summand_variables: HashMap<String, String>,
    pub user_variables: HashMap<String, String>,
    map: HashMap<String, String>,
    regex: Regex,
}

impl SummandWorkspace {
    pub fn new() -> Self {
        let mut workspace = SummandWorkspace {
            env_variables: HashMap::new(),
            summand_variables: HashMap::new(),
            user_variables: HashMap::new(),
            map: HashMap::new(),
            regex: Regex::from_str("").unwrap(),
        };
        workspace.detect_runtime();
        workspace
    }

    fn detect_runtime(&mut self) {
        let mut platform = "unix";
        if cfg!(target_os = "windows") {
            platform = "windows";
        } else if cfg!(target_os = "macos") {
            platform = "macos";
        }
        self.env_variables.insert("OS_NAME".to_string(), platform.to_string());
    }

    pub fn load_summand(&mut self, summand: &Summand) {
        self.summand_variables.clear();
        for item in &summand.variables {
            self.summand_variables.insert(
                item.name.clone(),
                item.value.clone().unwrap_or("".to_string()),
            );
        }
    }

    pub fn unload_summand(&mut self, summand: &Summand) {
        for item in &summand.variables {
            self.summand_variables.remove(&item.name);
        }
    }

    pub fn prepare(&mut self) {
        let mut merged = HashMap::new();
        merged.extend(
            self.summand_variables
                .iter()
                .map(|(k, v)| (format!("$${}", k), v.clone())),
        );
        merged.extend(
            self.user_variables
                .iter()
                .map(|(k, v)| (format!("$${}", k), v.clone())),
        );
        merged.extend(
            self.env_variables
                .iter()
                .map(|(k, v)| (format!("$${}", k), v.clone())),
        );
        self.map = merged;

        let pattern = self
            .map
            .keys()
            .map(|k| regex::escape(k)) // Escape special regex characters
            .collect::<Vec<String>>()
            .join("|"); // Create "key1|key2|key3" regex pattern

        self.regex = Regex::new(&pattern).unwrap();
    }

    pub fn fill(&self, text: &str) -> String {
        if self.map.is_empty() {
            return text.to_string();
        }
        self.regex
            .replace_all(text, |caps: &regex::Captures| {
                self.map.get(&caps[0]).unwrap().to_string()
            })
            .to_string()
    }
}

impl fmt::Display for SummandWorkspace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Summand Workspace(map: {:?})", self.map,)
    }
}
