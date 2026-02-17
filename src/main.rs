use std::{fs, path::PathBuf};

use clap::Parser;

mod interpreter;
mod lexer;
mod memory;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
enum Commands {
    Run { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { path } => {
            let source = fs::read_to_string(&path).unwrap();
            interpreter::run(&source);
        }
    }
}
