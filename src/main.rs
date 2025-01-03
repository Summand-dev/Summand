#[cfg(feature = "api-module")]
use api::server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Summand is starting ...");
    #[cfg(feature = "api-module")]
    {
        if let Err(e) = server::run().await {
            eprintln!("Api encountered an error: {}", e);
        }
    }
    // tokio::signal::ctrl_c()
    //     .await
    //     .expect("Failed to listen for shutdown signal");
    println!("Summand is shutting down gracefully...");
    Ok(())
}
