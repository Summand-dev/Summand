use std::env;

use crate::config::{
    adapters::json_config_adapter::JsonConfigAdapter, config::Config, config_adapter::ConfigAdapter,
};

use super::{
    dependency_injection::{DependencyInjector, Injectable},
    service_provider::ServiceProvider,
};

pub struct Application {
    pub providers: Vec<Box<dyn ServiceProvider>>,
    pub di: DependencyInjector,
    pub config: Config,
}

impl Application {
    pub fn new(service_provider: Vec<Box<dyn ServiceProvider>>, di: DependencyInjector) -> Self {
        Application {
            providers: service_provider,
            di: di,
            config: Config {
                ..Default::default()
            },
        }
    }

    pub fn boot(&mut self) {
        let dir = env::current_dir();
        match dir {
            Ok(d) => {
                println!("Application run dir: {}", d.display());
            }
            Err(_) => todo!(),
        }
        let config_adapter = JsonConfigAdapter::new();
        let result = config_adapter.load("config.json");
        match result {
            Ok(config) => {
                self.config = config;
            }
            Err(_) => {
                println!("Failed to load config");
            }
        }
        let providers = std::mem::take(&mut self.providers);
        for provider in providers {
            provider.as_ref().boot(self);
        }
    }
}
