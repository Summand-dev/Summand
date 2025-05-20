use summand_data::{adapters::sqlx_data_adapter::SQLxDataAdapter, data_adapter::DataAdapter};


#[derive(clap::Args, Clone, Debug)]
pub struct ListArgs {
    #[arg()]
    name: Option<String>,
}

pub async fn handle(args: ListArgs) -> anyhow::Result<()> {
    let data = SQLxDataAdapter::new().await;
    if let Some(name) = args.name {
        let summand = data.find(name).await;
        match summand {
            Some(summand) => {
                println!("ID: {}", summand.id);
                println!("Name: {}", summand.name);
                println!();
            }
            None => {
                println!("Summand not found.");
            }
        }
    } else {
        let summands = data.find_all().await;
        if summands.len() == 0 {
            println!("No summands found.");
            return Ok(());
        }
        for summand in &summands {
            println!("ID: {}", summand.id);
            println!("Name: {}", summand.name);
            println!();
        }
        println!("Total: {}", summands.len());
    }
    Ok(())
}
