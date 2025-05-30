use super::{
    dependency_injection::{DependencyInjector, Injectable},
    service_provider::ServiceProvider,
};

pub struct Application {
    pub providers: Vec<Box<dyn ServiceProvider>>,
    pub di: DependencyInjector,
}

impl Application {
    pub fn new(service_provider: Vec<Box<dyn ServiceProvider>>, di: DependencyInjector) -> Self {
        Application {
            providers: service_provider,
            di: di,
        }
    }

    pub fn boot(&mut self) {
        let providers = std::mem::take(&mut self.providers);
        for provider in providers {
            provider.as_ref().boot(self);
        }
    }
}
