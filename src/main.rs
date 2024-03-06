use clap::{Parser, Subcommand};
use net::Todo;
use std::path::PathBuf;
use std::{error, result};

// net init (create new)
// net todo.txt (parse)

type Result<T> = result::Result<T, Box<dyn error::Error>>;

// macro_rules! err {
//     ($($tt:tt)*) => { Err(Box::<dyn error::Error>::from(format!($($tt)*))) };
// }

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
    /// Parse existing TODO file
    Parse { todo_file: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            println!("Creating new todo file");
            let todo = Todo::new()?;
            todo.save()?;
            Ok(())
        }
        Commands::Parse { todo_file } => {
            // get last day
            // if last day is before today: create today
            // if last day is after today: what the fuck?
            println!("parsing {:?}", todo_file);
            Ok(())
        }
    }
}
