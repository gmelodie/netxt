use chrono::{Local, NaiveDate};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{error, result};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error>::from(format!($($tt)*))) };
}

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn today() -> NaiveDate {
    // Current local time
    let now = Local::now();

    // Current local date
    now.date_naive()
}

pub fn get_last_day(todo_file: &PathBuf) -> Result<NaiveDate> {
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
