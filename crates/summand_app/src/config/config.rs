use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub database: Option<DatabaseConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DatabaseConfig {
    pub adapter: String,
    pub path: Option<String>,
}
