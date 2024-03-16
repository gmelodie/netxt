//! Todo is NOT timezone-aware

use itertools::Itertools;
use section::Section;
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

pub use util::today;
use util::*;

pub use day::{Day, DayIterator};
use task::Task;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn error::Error>::from(format!($($tt)*))) };
}

static DEFAULT_TODO_FILE: &str = "todo.txt";

#[derive(PartialEq, Debug, Clone)]
pub struct Todo<'todo_life> {
    pub today: Day,
    pub days: Vec<Day>, // does not contain today, only contains days that are finished
    file_path: &'todo_life Path,
}

impl<'todo_life> Todo<'todo_life> {
    pub fn new(path: Option<&'todo_life str>) -> Result<Todo<'todo_life>> {
        let path = match path {
            // if path present but file doesnt exist, create it
            Some(path) => {
                if let Err(_error) = OpenOptions::new().read(true).open(path) {
                    OpenOptions::new().write(true).create(true).open(path)?;
                };
                Path::new(path)
            }
            // if path not present, create default file if possible
            None => {
                if let Err(_error) = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(DEFAULT_TODO_FILE)
                {
                    return err!("File with default name {DEFAULT_TODO_FILE} already exists");
                }
                Path::new(DEFAULT_TODO_FILE)
            }
        };

        // load from file or create new blank one
        let todo = Todo::load(path).unwrap_or(Todo {
            today: Day::new(today()),
            days: Vec::<Day>::new(),
            file_path: &Path::new(path),
        });
        Ok(todo)
    }

    /// Saves today in days and creates new today
    pub fn next_day(&mut self) {
        self.days.push(self.today.clone());
        self.today.date = today();
        // clear "Done" section, create one if didnt find
        match self
            .today
            .sections
            .iter()
            .position(|section| section.name == "Done")
        {
            Some(pos) => {
                self.today.sections[pos].tasks = Vec::<Task>::new();
            }
            None => {
                let sec = Section::new("Done");
                self.today.sections.push(sec);
            }
        }
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
            return err!("File already up to date");
        }

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
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

    pub fn add(&mut self, task_txt: &str, section: &str) -> Result<()> {
        let task: Task = task_txt.parse()?;
        let sections: &mut Vec<Section> = &mut self.today.sections;

        // find section position in vec
        let pos = sections
            .iter()
            .position(|sec| sec.name == section)
            // create section if it doesnt exist
            .unwrap_or_else(|| {
                sections.push(Section::new(section));
                sections.len() - 1
            });

        // put task in section
        sections[pos].tasks.push(task);
        Ok(())
    }
}

impl<'a> str::FromStr for Todo<'a> {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let text = s.trim().to_string();
        let mut days: Vec<Day> = Vec::new();
        let day_iter = DayIterator::new(&text);
        let mut cur_day = Day::new(today());
        for day in day_iter {
            if day.date == today() {
                cur_day = day;
            } else {
                days.push(day);
            }
        }
        Ok(Todo {
            today: cur_day,
            days,
            file_path: &Path::new(""), // no path to give, is this an issue?
        })
    }
}

impl<'a> fmt::Display for Todo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let days = self.days.iter().join("\n");
        write!(f, "{days}\n{}", self.today)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::section::Section;
    use crate::task::Task;

    use chrono::Duration as ChronoDuration;
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

    #[test]
    fn add_task() {
        let base = Todo {
            today: Day {
                date: today(),
                sections: vec![
                    Section {
                        name: "Section 1".to_string(),
                        tasks: vec![
                            Task {
                                text: "task 1".to_string(),
                            },
                            Task {
                                text: "task 2".to_string(),
                            },
                            Task {
                                text: "task 3".to_string(),
                            },
                        ],
                    },
                    Section {
                        name: "Done".to_string(),
                        tasks: vec![],
                    },
                ],
            },
            file_path: Path::new(""),
            days: vec![],
        };

        let expected = Todo {
            today: Day {
                date: today(),
                sections: vec![
                    Section {
                        name: "Section 1".to_string(),
                        tasks: vec![
                            Task {
                                text: "task 1".to_string(),
                            },
                            Task {
                                text: "task 2".to_string(),
                            },
                            Task {
                                text: "task 3".to_string(),
                            },
                            Task {
                                text: "added task".to_string(),
                            },
                        ],
                    },
                    Section {
                        name: "Done".to_string(),
                        tasks: vec![],
                    },
                ],
            },
            file_path: Path::new(""),
            days: vec![],
        };

        let mut actual = base.clone();
        actual.add("added task", "Section 1").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_task_new_section() {
        let base = Todo {
            today: Day {
                date: today(),
                sections: vec![
                    Section {
                        name: "Section 1".to_string(),
                        tasks: vec![
                            Task {
                                text: "task 1".to_string(),
                            },
                            Task {
                                text: "task 2".to_string(),
                            },
                            Task {
                                text: "task 3".to_string(),
                            },
                        ],
                    },
                    Section {
                        name: "Done".to_string(),
                        tasks: vec![],
                    },
                ],
            },
            file_path: Path::new(""),
            days: vec![],
        };

        let expected = Todo {
            today: Day {
                date: today(),
                sections: vec![
                    Section {
                        name: "Section 1".to_string(),
                        tasks: vec![
                            Task {
                                text: "task 1".to_string(),
                            },
                            Task {
                                text: "task 2".to_string(),
                            },
                            Task {
                                text: "task 3".to_string(),
                            },
                        ],
                    },
                    Section {
                        name: "Done".to_string(),
                        tasks: vec![],
                    },
                    // new section is added to end of vec, not before Done
                    Section {
                        name: "New Section".to_string(),
                        tasks: vec![Task {
                            text: "added task".to_string(),
                        }],
                    },
                ],
            },
            file_path: Path::new(""),
            days: vec![],
        };

        let mut actual = base.clone();
        actual.add("added task", "New Section").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn next_day() {
        let base_day = today();
        let base = Todo {
            today: Day {
                date: base_day,
                sections: vec![
                    Section {
                        name: "Section 1".to_string(),
                        tasks: vec![
                            Task {
                                text: "task 1".to_string(),
                            },
                            Task {
                                text: "task 2".to_string(),
                            },
                        ],
                    },
                    Section {
                        name: "Done".to_string(),
                        tasks: vec![Task {
                            text: "task 3".to_string(),
                        }],
                    },
                ],
            },
            file_path: Path::new(""),
            days: vec![],
        };

        let expected = Todo {
            today: Day {
                date: base_day + ChronoDuration::days(1),
                sections: vec![
                    Section {
                        name: "Section 1".to_string(),
                        tasks: vec![
                            Task {
                                text: "task 1".to_string(),
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
            file_path: Path::new(""),
            days: vec![Day {
                date: base_day,
                sections: vec![
                    Section {
                        name: "Section 1".to_string(),
                        tasks: vec![
                            Task {
                                text: "task 1".to_string(),
                            },
                            Task {
                                text: "task 2".to_string(),
                            },
                        ],
                    },
                    Section {
                        name: "Done".to_string(),
                        tasks: vec![Task {
                            text: "task 3".to_string(),
                        }],
                    },
                ],
            }],
        };

        let mut actual = base.clone();
        // need to manually set the date
        // TODO: find a good way to mock time

        actual.next_day();
        actual.today.date = base_day + ChronoDuration::days(1);
        assert_eq!(actual, expected);
    }
}
