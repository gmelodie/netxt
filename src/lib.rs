//! Todo is NOT timezone-aware

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

mod day;
mod section;
mod task;
mod util;

use util::*;

use day::Day;

// TODO: implement Read and Write for Todo

#[derive(PartialEq, Debug)]
pub struct Todo {
    today: Day,
    days: Vec<Day>, // does not contain today, only contains days that are finished
    file_path: PathBuf,
}

impl Todo {
    pub fn new(path: &str) -> Result<Todo> {
        let todo = Todo {
            today: Day::new(today()),
            days: Vec::<Day>::new(),
            file_path: path.into(),
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
        Ok(Todo::new(todo_file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::section::Section;
    use crate::task::Task;

    use chrono::NaiveDate;
    use indoc::indoc;
    use std::fs::read_to_string;
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
        let path = file.path().to_path_buf();
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

        let actual2: Todo = read_to_string(&path)
            .expect("Unable to read file")
            .parse()
            .expect("Unable to parse file contents");
        assert_eq!(actual2, expected);
    }
}

//TODO: a day can have tasks that are in anonymous section
