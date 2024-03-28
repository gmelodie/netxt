use chrono::{Local, NaiveDate};
use std::error;
use std::fs::read_to_string;
use std::path::Path;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

pub fn today() -> NaiveDate {
    // Current local time
    let now = Local::now();

    // Current local date
    now.date_naive()
}

pub fn get_last_day(todo_file: &Path) -> Option<NaiveDate> {
    let contents = read_to_string(todo_file).expect("Unable to read file get_last_day");
    for line in contents.lines().rev() {
        if let Some(first_char) = line.chars().next() {
            if first_char == '[' {
                if let Ok(date) = NaiveDate::parse_from_str(line, "[%Y-%m-%d]") {
                    return Some(date);
                };
            }
        }
    }
    None
}
