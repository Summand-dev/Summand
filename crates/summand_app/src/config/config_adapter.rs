use crate::config::config::Config;

pub trait ConfigAdapter {
    fn new() -> Self;
    fn load(&self, path: &str) -> Result<Config, String>;
}
