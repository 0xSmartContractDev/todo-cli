use anyhow::Result;
use clap::Parser;
use todo_cli::{cli::Cli, store}; // Needed for Cli::parse()

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = cli.file.unwrap_or_else(store::default_path);
    println!("Using data file: {}", path.display());
    Ok(())
}
