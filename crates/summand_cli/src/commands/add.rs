use summand_data::{adapters::sqlx_data_adapter::SQLxDataAdapter, data_adapter::DataAdapter};



#[derive(clap::Args, Clone, Debug)]
pub struct AddArgs {
    #[arg()]
    name: String,
    #[arg(long = "description", short = 'D', value_name = "DESCRIPTION")]
    description: Option<String>,
    #[arg(long = "command", value_name = "COMMAND", num_args = 1.. )]
    command: Option<Vec<String>>,
}

pub async fn handle(args: AddArgs) -> anyhow::Result<()> {

    let command = args.command.unwrap();

    // println!("args : {:#?}", args.command);

    println!("command : {:#?}", command);
    
    let command = summand_app::command::Command::new(
        "Echo",
        Some("Echo a fixed value"),
        command[0].as_str(),
        Some(vec![summand_app::argument::CommandArgument::new(
            None,
            Some(command[1].as_str()),
        )]),
    );

    let summand = summand_app::summand::Summand::new(
        &args.name,
        args.description.as_deref(),
        Some(vec![command]),
    );

    let data = SQLxDataAdapter::new().await;

    print!("data : {:#?}", data);
    data.create(&summand).await;

    Ok(())
}
