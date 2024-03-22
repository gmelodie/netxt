//! Todo is NOT timezone-aware

use chrono::NaiveDate;
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
    pub days: Vec<Day>,
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
            days: Vec::<Day>::new(),
            file_path: &Path::new(path),
        });

        Ok(todo)
    }

    fn last_day(&self) -> Option<&Day> {
        if self.days.len() == 0 {
            return None;
        }

        let mut last_day = &self.days[0];
        for day in &self.days {
            if day.date > last_day.date {
                last_day = &day;
            }
        }
        Some(last_day)
    }

    fn last_day_pos(&self) -> Option<usize> {
        let last_day = self.last_day()?;
        Some(
            self.days
                .iter()
                .position(|day| day.date == last_day.date)
                // create section if it doesnt exist
                .expect("Unable to find last_day pos"),
        )
    }

    /// Creates new day with all tasks/sections from most recent day and cleared Done section
    /// next_day is idempotent, meaning it will do nothing if today already exists in days
    pub fn next_day(&mut self) {
        let mut new_day = match self.last_day() {
            Some(last_day) => {
                // days and today is created: do nothing
                if last_day.date == today() {
                    return;
                }
                // days but no today: copy last day
                let mut clone_last_day = last_day.clone();
                clone_last_day.date = today();
                clone_last_day
            }
            // no days: create new empty day
            None => Day::new(today()),
        };

        // clear "Done" section, create one if didnt find
        match new_day
            .sections
            .iter()
            .position(|section| section.name == "Done")
        {
            Some(pos) => {
                new_day.sections[pos].tasks = Vec::<Task>::new();
            }
            None => {
                let sec = Section::new("Done");
                new_day.sections.push(sec);
            }
        }

        self.days.push(new_day);
    }

    pub fn save(&mut self) -> Result<()> {
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

        // merge todo into file if file already exists
        if self.file_path.exists() {
            let file_todo = Todo::new(Some(
                self.file_path.to_str().expect("Could not transform path"),
            ))?;
            for day in file_todo.days {
                if !self.days.contains(&day) {
                    self.days.push(day.clone());
                }
            }
            self.days.sort();
        }

        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
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
        // make sure current day exists
        self.next_day();

        let task: Task = task_txt.parse()?;
        let day_pos = self.last_day_pos().expect("Could not get last day pos");

        // find section position in vec
        let sections = &mut self.days[day_pos].sections;

        let section_pos = sections
            .iter()
            .position(|sec| sec.name == section)
            // create section if it doesnt exist
            .unwrap_or_else(|| {
                sections.push(Section::new(section));
                sections.len() - 1
            });

        // put task in section
        self.days[day_pos].sections[section_pos].tasks.push(task);
        Ok(())
    }
}

impl<'a> str::FromStr for Todo<'a> {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let text = s.trim().to_string();
        let mut days: Vec<Day> = Vec::new();
        let day_iter = DayIterator::new(&text);

        let old_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let mut last_day = Day::new(old_date); // set date to old date

        for day in day_iter {
            if day.date > last_day.date {
                last_day = day.clone();
            }
            days.push(day);
        }

        Ok(Todo {
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
            - task 4
        "}
            .to_string(),
        );
        let path = file.path();
        let expected = Todo {
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
                            tasks: vec![Task {
                                text: "task 4".to_string(),
                            }],
                        },
                    ],
                },
            ],
            file_path: path,
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
        let mut todo = Todo {
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
            days: vec![Day {
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
            }],
            file_path: Path::new(""),
        };

        let expected = Todo {
            days: vec![Day {
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
            }],
            file_path: Path::new(""),
        };

        let mut actual = base.clone();
        actual.add("added task", "Section 1").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_task_new_section() {
        let base = Todo {
            days: vec![Day {
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
            }],
            file_path: Path::new(""),
        };

        let expected = Todo {
            days: vec![Day {
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
            }],
            file_path: Path::new(""),
        };

        let mut actual = base.clone();
        actual.add("added task", "New Section").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn next_day() {
        let base = Todo {
            days: vec![Day {
                date: today() - ChronoDuration::days(1),
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
            file_path: Path::new(""),
        };

        let expected = Todo {
            days: vec![
                Day {
                    date: today() - ChronoDuration::days(1),
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
                Day {
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
                            ],
                        },
                        Section {
                            name: "Done".to_string(),
                            tasks: vec![],
                        },
                    ],
                },
            ],
            file_path: Path::new(""),
        };

        let mut actual = base.clone();

        actual.next_day();
        assert_eq!(actual, expected);
    }
}
