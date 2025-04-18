#[cfg(feature = "api-module")]
use summand_api::server;

#[cfg(feature = "cli-module")]
use summand_cli::app::Cli;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Summand is starting ...");
    #[cfg(feature = "api-module")]
    {
        server::run();
    }
    #[cfg(feature = "cli-module")]
    {
        Cli::init().run();
    }
    // tokio::signal::ctrl_c()
    //     .await
    //     .expect("Failed to listen for shutdown signal");
    println!("Summand is shutting down gracefully...");
    Ok(())
}
