use summand_app::app::application::Application;

pub async fn handle(app: &Application) -> anyhow::Result<()> {
    println!("Version 1.0.0");
    println!("Config Loaded in version : {:?}", app.config);
    Ok(())
}
