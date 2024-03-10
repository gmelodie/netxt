use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::{error, result};

use netxt::Todo;

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
    Add {
        todo_file: PathBuf,
        task: String,
        section: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            let todo = Todo::new(None)?;
            todo.save()?;
            Ok(())
        }
        Commands::Parse { todo_file } => {
            let _todo = Todo::load(todo_file)?;
            Ok(())
        }
        Commands::Add {
            todo_file,
            task,
            section,
        } => {
            let section = section.clone().unwrap_or("".to_string());
            let mut todo = Todo::load(todo_file)?;
            todo.add(&task, &section)?;
            Ok(())
        }
    }
}
