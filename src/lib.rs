use chrono::{Local, NaiveDate};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

/// Todo is NOT timezone-aware
pub struct Todo {
    last_day: NaiveDate, // last day
    file_path: PathBuf,  // last day
}

// Local::now().format("%d-%m-%Y")
// parse_from_str(string, "%d-%m-%Y")

fn today() -> NaiveDate {
    // Current local time
    let now = Local::now();

    // Current local date
    now.date_naive()
}
impl Todo {
    pub fn new() -> Result<Todo> {
        let todo = Todo {
            last_day: today(),
            file_path: env::current_dir()?.join("todo.txt"),
        };
        Ok(todo)
    }

    pub fn save(&self) -> Result<()> {
        // TODO: check if file is up to date, if so, don't save
        let mut f = if self.file_path.exists() {
            File::open(&self.file_path)?
        } else {
            // if file doesn't exist, create it
            File::create(&self.file_path)?
        };
        // TODO: implement Display for NaiveDate
        // TODO: implement Read
        f.write_all(self.last_day.format("[%d %m %Y]").to_string().as_bytes())?;
        Ok(())
    }
}
