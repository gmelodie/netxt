use clap::{Parser, Subcommand};
use std::path::PathBuf;

// net init (create new)
// net todo.txt (parse)

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Existing TODO file to parse
    #[arg(short, long, value_name = "FILE")]
    todo_file: Option<PathBuf>,

    #[arg(short, long)]
    init: Option<bool>,
}

fn main() {
    let cli = Cli::parse();
    println!("Never ending TXT");
}
