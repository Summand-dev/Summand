use std::path::PathBuf;
use summand_app::core::SummandRunner;
use summand_data::{adapters::sqlx_data_adapter::SQLxDataAdapter, data_adapter::DataAdapter};
// use summand_app::{core::SummandRunner};

#[derive(clap::Args, Clone, Debug)]
pub struct RunArgs {
    #[arg()]
    summand: String,
}

pub async fn handle(args: RunArgs) -> anyhow::Result<()> {

    let summand_name = args.summand;

    let database = SQLxDataAdapter::new().await;

    let mut runner = SummandRunner::new();

    let summand_data = database.find(summand_name.clone()).await;

    if summand_data.is_none() {
        return Err(anyhow::anyhow!("Summand not found: {}", summand_name));
    }

    runner.add_summand(&summand_data.unwrap());
    runner.execute_next().await;

    print!("summand: {:?}", &summand_name);

    Ok(())
}
