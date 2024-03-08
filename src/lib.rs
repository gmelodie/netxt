//! Todo is NOT timezone-aware

use itertools::Itertools;
use std::error;
use std::fmt;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str;

mod day;
mod section;
mod task;
mod util;

use util::*;

use day::{Day, DayIterator};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error>::from(format!($($tt)*))) };
}

#[derive(PartialEq, Debug)]
pub struct Todo<'a> {
    today: Day,
    days: Vec<Day>, // does not contain today, only contains days that are finished
    file_path: &'a Path,
}

impl<'a> Todo<'a> {
    pub fn new(path: &str) -> Result<Todo<'a>> {
        let todo = Todo {
            today: Day::new(today()),
            days: Vec::<Day>::new(),
            file_path: &Path::new(""),
        };
        Ok(todo)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(last_day_in_file) = get_last_day(self.file_path) {
            if last_day_in_file > today() {
                return err!("Invalid date: date on file is ahead of today");
            }
        }

        // don't save if file is up to date
        let file_todo = Todo::load(self.file_path)?;
        if file_todo == *self {
            return Ok(()); // maybe want to return Err here?
        }

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        f.write_all(format!("{self}").as_bytes())?;
        Ok(())
    }

    pub fn load(todo_file: &Path) -> Result<Todo> {
        if let Some(last_day) = get_last_day(todo_file) {
            if last_day > today() {
                return err!("Invalid date: date on file is ahead of today");
            }
        }
        let mut todo: Todo = read_to_string(&todo_file)
            .expect("Unable to read file")
            .parse()
            .expect("Unable to parse file contents");
        todo.file_path = todo_file;
        Ok(todo)
    }
}

impl<'a> str::FromStr for Todo<'a> {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let text = s.trim().to_string();
        let mut days: Vec<Day> = Vec::new();
        let day_iter = DayIterator::new(&text);
        for day in day_iter {
            days.push(day);
        }
        Ok(Todo {
            today: Day::new(today()),
            days,
            file_path: &Path::new(""), // no path to give, is this an issue?
        })
    }
}

impl<'a> fmt::Display for Todo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let days = self.days.iter().join("\n");
        write!(f, "{days}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::section::Section;
    use crate::task::Task;

    use chrono::NaiveDate;
    use indoc::indoc;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Creates a tmp file with string contents and return the file path
    fn create_file_with_contents(contents: String) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Unable to create tmp file");
        file.write_all(contents.as_bytes())
            .expect("Unable to write to tmp file");
        file
    }

    #[test]
    fn parse_todo() {
        let file = create_file_with_contents(
            indoc! {"
            [2024-03-06]
            Section 1
            - task 1
            - task 3
            - task 2
            Done

            [2024-03-07]
            Section 2
            - task 11
            - task 31
            - task 21
            Done
        "}
            .to_string(),
        );
        let path = file.path();
        let expected = Todo {
            today: Day::new(today()),
            file_path: path,
            days: vec![
                Day {
                    date: NaiveDate::from_ymd_opt(2024, 3, 6).unwrap(),
                    sections: vec![
                        Section {
                            name: "Section 1".to_string(),
                            tasks: vec![
                                Task {
                                    text: "task 1".to_string(),
                                },
                                Task {
                                    text: "task 3".to_string(),
                                },
                                Task {
                                    text: "task 2".to_string(),
                                },
                            ],
                        },
                        Section {
                            name: "Done".to_string(),
                            tasks: vec![],
                        },
                    ],
                },
                Day {
                    date: NaiveDate::from_ymd_opt(2024, 3, 7).unwrap(),
                    sections: vec![
                        Section {
                            name: "Section 2".to_string(),
                            tasks: vec![
                                Task {
                                    text: "task 11".to_string(),
                                },
                                Task {
                                    text: "task 31".to_string(),
                                },
                                Task {
                                    text: "task 21".to_string(),
                                },
                            ],
                        },
                        Section {
                            name: "Done".to_string(),
                            tasks: vec![],
                        },
                    ],
                },
            ],
        };

        let actual = Todo::load(&path).expect("Unable to load file");
        assert_eq!(actual, expected);
    }

    #[test]
    fn save_todo() {
        let expected = indoc! {"
            [2024-03-06]
            Section 1
            - task 1
            - task 3
            - task 2
            Done

            [2024-03-07]
            Section 2
            - task 11
            - task 31
            - task 21
            Done
        "};

        let file = NamedTempFile::new().expect("Unable to create tmp file");
        let path = file.path();
        let todo = Todo {
            today: Day::new(today()),
            file_path: path,
            days: vec![
                Day {
                    date: NaiveDate::from_ymd_opt(2024, 3, 6).unwrap(),
                    sections: vec![
                        Section {
                            name: "Section 1".to_string(),
                            tasks: vec![
                                Task {
                                    text: "task 1".to_string(),
                                },
                                Task {
                                    text: "task 3".to_string(),
                                },
                                Task {
                                    text: "task 2".to_string(),
                                },
                            ],
                        },
                        Section {
                            name: "Done".to_string(),
                            tasks: vec![],
                        },
                    ],
                },
                Day {
                    date: NaiveDate::from_ymd_opt(2024, 3, 7).unwrap(),
                    sections: vec![
                        Section {
                            name: "Section 2".to_string(),
                            tasks: vec![
                                Task {
                                    text: "task 11".to_string(),
                                },
                                Task {
                                    text: "task 31".to_string(),
                                },
                                Task {
                                    text: "task 21".to_string(),
                                },
                            ],
                        },
                        Section {
                            name: "Done".to_string(),
                            tasks: vec![],
                        },
                    ],
                },
            ],
        };

        let _ = todo.save().expect("Unable to load file");

        let actual = read_to_string(&path).expect("Unable to read file");
        assert_eq!(actual, expected);
    }
}
