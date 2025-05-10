use std::path::Path;

#[derive(clap::Args, Clone, Debug)]
#[command(verbatim_doc_comment)]
pub(crate) struct FixArgs {
    #[arg(long = "import", value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
    app_import: Option<PathBuf>,
}

pub fn handle() -> anyhow::Result<()> {
    println!("Version 1.0.0");
    Ok(())
}
