use clap::{Parser, Subcommand};
use net::Todo;
use std::path::PathBuf;
use std::{error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new todo file
    Init,
    /// Parse existing todo file
    Parse { todo_file: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            let todo = Todo::new()?;
            todo.save()?;
            Ok(())
        }
        Commands::Parse { todo_file } => {
            let _todo = Todo::load(todo_file)?;
            Ok(())
        }
    }
}
