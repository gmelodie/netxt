use chrono::{Local, NaiveDate};
use std::env;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{error, result};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn error::Error>>;

/// Todo is NOT timezone-aware
// TODO: implement Read and Write for Todo
// Local::now().format("[%Y-%m-%d]")
// parse_from_str(string, "[%Y-%m-%d]")
pub struct Todo {
    last_day: NaiveDate, // last day
    file_path: PathBuf,  // last day
}

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
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        // TODO: implement Display for NaiveDate
        // TODO: check if current date is already present in file
        f.write_all(self.last_day.format("[%Y-%m-%d]\n").to_string().as_bytes())
            .unwrap();
        Ok(())
    }

    pub fn load(todo_file: &PathBuf) -> Result<Todo> {
        let last_day = get_last_day(todo_file)?;
        if last_day > today() {
            return err!("Invalid date: date on file is ahead of today");
        }
        Ok(Todo {
            last_day,
            file_path: todo_file.into(),
        })
    }
}

fn get_last_day(todo_file: &PathBuf) -> Result<NaiveDate> {
    for line in read_to_string(todo_file)?.lines().rev() {
        if let Some(first_char) = line.chars().next() {
            if first_char == '[' {
                let date = NaiveDate::parse_from_str(line, "[%Y-%m-%d]\n")?;
                return Ok(date);
            }
        }
    }
    err!("No lines matching date sytax")
}
