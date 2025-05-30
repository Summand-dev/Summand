use std::{default, fs};

use summand_app::app::{
    application::Application,
    dependency_injection::{DependencyInjector, InjectionType},
    service_provider::ServiceProvider,
};

#[derive(Debug)]
pub struct Database {
    pub url: String,
}

impl Database {
    pub fn new() -> Self {
        println!("initiator run");
        Database {
            url: "postgres://localhost".to_string(),
        }
    }
}

struct DatabaseProvider {}
impl ServiceProvider for DatabaseProvider {
    fn boot(&self, app: &mut Application) {
        app.di.bind("db", InjectionType::Singletone, Database::new);
    }
}

#[test]
fn run_app() {
    let mut app = Application::new(
        vec![Box::from(DatabaseProvider {})],
        DependencyInjector::default(),
    );
    app.boot();

    // Use Database as the same type here
    let db = app.di.resolve::<Database>("db");

    println!("Resolved: {:?}", db);
    assert_eq!(db.unwrap().url, "postgres://localhost");
}
