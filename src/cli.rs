use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub file: Option<std::path::PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        #[arg(short, long)]
        notes: Option<String>,
    },
    List {
        all: bool,
    },
    Done {
        id: u64,
    },
    Remove {
        id: u64,
    },
}
