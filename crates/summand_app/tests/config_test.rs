use std::env;

use summand_app::{
    app::{
        application::Application,
        dependency_injection::{DependencyInjector, InjectionType},
        service_provider::ServiceProvider,
    },
    config::{adapters::json_config_adapter::JsonConfigAdapter, config_adapter::ConfigAdapter},
};

struct ConfigProvider {}
impl ServiceProvider for ConfigProvider {
    fn boot(&self, app: &mut Application) {
        app.di
            .bind("config", InjectionType::Singletone, JsonConfigAdapter::new);
    }
}

#[test]
fn run_app() {
    let mut app = Application::new(
        vec![Box::from(ConfigProvider {})],
        DependencyInjector::default(),
    );
    app.boot();

    println!("App config loaded: {:?}", app.config);

    // Use Database as the same type here
    // let maybe_config = app.di.resolve::<JsonConfigAdapter>("config");
    // if let Some(config) = maybe_config {
    //     config.load("../../config.json");
    //     println!("Config: {:?}", config);
    // }

    // assert_eq!(maybe_config.unwrap().config, "postgres://localhost");
}
