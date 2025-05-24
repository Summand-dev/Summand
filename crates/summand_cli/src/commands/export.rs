use std::path::PathBuf;
use summand_data::{adapters::sqlx_data_adapter::SQLxDataAdapter, data_adapter::DataAdapter};

#[derive(clap::Args, Clone, Debug)]
pub struct ExportArgs {
    #[arg()]
    destination: PathBuf,
}

pub async fn handle(args: ExportArgs) -> anyhow::Result<()> {
    let destination = args.destination;

    let database = SQLxDataAdapter::new().await;

    if destination.extension() == Some(std::ffi::OsStr::new("json")) {
        
        let data = database.find_all().await;
        if let Err(e) = std::fs::write(&destination, serde_json::to_string(&data)?) {
            return Err(anyhow::anyhow!("Failed to write to file: {}", e));
        }

        println!("Data exported successfully to {}", &destination.display());
    } else {
        return Err(anyhow::anyhow!(
            "Destination file must have a .json extension"
        ));
    }

    Ok(())
}
