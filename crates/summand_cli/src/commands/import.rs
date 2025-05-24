use std::{fs, path::Path};
use summand_data::{adapters::sqlx_data_adapter::SQLxDataAdapter, data_adapter::DataAdapter};
use summand_parser::parser::SummandParser;
use url::Url;

#[derive(clap::Args, Clone, Debug)]
#[command(verbatim_doc_comment)]
pub struct ImportArgs {
    #[arg()]
    source: String,
}

pub async fn handle(args: ImportArgs) -> anyhow::Result<()> {
    let source = &args.source;
    
    let is_url = Url::parse(source);

    if is_url.is_ok() {
        println!("Source url is: {}", source);
    } 
    else if Path::new(source).exists() {
        let file_path = Path::new(source);
        if file_path.is_file() {
            println!("Your summands are being imported from: {}", source);
            let summand_json = fs::read_to_string(file_path).expect("Unable to read file");

            let parser = SummandParser::new();
            let database = SQLxDataAdapter::new().await;
            let summand = parser.parse(summand_json.as_str());
            
            database.create(&summand).await;
            println!("Summand Id {}", summand.id);
            println!("Summand Name {}", summand.name);
        }
        else{
            print!("Your source path is invalid: {}", source);
        }
    }
    else{
        println!("Invalid source: {}", source);
    }
    Ok(())
}
