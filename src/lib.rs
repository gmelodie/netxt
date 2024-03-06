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

struct SectionIterator<'a> {
    lines: Vec<&'a str>,
    index: usize,
}

impl<'a> SectionIterator<'a> {
    fn new(s: &'a str) -> Self {
        SectionIterator {
            lines: s.lines().collect(),
            index: 0,
        }
    }
}

impl<'a> Iterator for SectionIterator<'a> {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        let mut section: Vec<&'a str> = Vec::new();
        if self.index >= self.lines.len() {
            None
        } else {
            // first line must be start of a section, consume anything that is not start of section
            while self.index < self.lines.len() && !is_section_start(self.lines[self.index]) {
                self.index += 1;
            }
            if self.index >= self.lines.len() {
                return None;
            }

            // read start of section
            let start = self.lines[self.index];
            section.push(start);
            self.index += 1;

            // read everything until start of next section
            while self.index < self.lines.len() && !is_section_start(self.lines[self.index]) {
                section.push(self.lines[self.index]);
                self.index += 1;
            }

            Some(Section::new(&section.join("\n")))
        }
    }
}

fn is_section_start(line: &str) -> bool {
    if line.trim().is_empty() {
        return false;
    }
    let first_char = line.chars().next();
    match first_char {
        Some('-') => false,
        Some('[') => false,
        None => false,
        _ => true,
    }
}

#[derive(PartialEq, Debug)]
struct Day {
    date: NaiveDate,
    sections: Vec<Section>,
}

impl Day {
    fn new(text: &str) -> Day {
        let text = text.trim().to_string();
        // first line must be the date
        let mut lines = text.lines();
        let date = NaiveDate::parse_from_str(lines.next().unwrap().trim(), "[%Y-%m-%d]")
            .expect("Unable to parse date"); // TODO: return error if this is not a date

        let mut sections: Vec<Section> = Vec::new();
        // the rest of the lines should be sections with tasks
        let section_iter = SectionIterator::new(&text);
        for section in section_iter {
            sections.push(section);
        }
        Day { date, sections }
    }
}

#[derive(PartialEq, Debug)]
pub struct Section {
    name: String,
    tasks: Vec<Task>,
}

impl Section {
    fn new(text: &str) -> Section {
        let text = text.trim().to_string();
        // first line must be the section name
        let mut lines = text.lines();
        let name = lines
            .next()
            .expect("Unable to read section name")
            .trim()
            .to_string(); // TODO: return error if this is not a section name

        let mut tasks: Vec<Task> = Vec::new();
        // the rest of the lines should be tasks
        for line in lines {
            tasks.push(Task::new(line));
        }
        Section { name, tasks }
    }
}

#[derive(PartialEq, Debug)]
struct Task {
    text: String,
}

impl Task {
    fn new(text: &str) -> Task {
        Task {
            text: text.trim_start_matches(&['-', ' ']).to_string(), // trim - and spaces
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_task() {
        let task_line = "- pick up dry cleaning";
        let actual = Task::new(task_line);
        let expected = Task {
            text: "pick up dry cleaning".to_string(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_section() {
        let section_text = indoc! {"
            Some Section
            - task 1
            - task 3
            - task 2

        "};

        let actual = Section::new(section_text);
        let expected = Section {
            name: "Some Section".to_string(),
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
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_day() {
        let day_text = indoc! {"
            [2024-03-06]
            Section 1
            - task 1
            - task 3
            - task 2

            Section 2
            - task 2.1
            - task 2.2
            Section 3
            - task 3.2
            - task 3.1
        "};

        let actual = Day::new(day_text);
        let expected = Day {
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
                    name: "Section 2".to_string(),
                    tasks: vec![
                        Task {
                            text: "task 2.1".to_string(),
                        },
                        Task {
                            text: "task 2.2".to_string(),
                        },
                    ],
                },
                Section {
                    name: "Section 3".to_string(),
                    tasks: vec![
                        Task {
                            text: "task 3.2".to_string(),
                        },
                        Task {
                            text: "task 3.1".to_string(),
                        },
                    ],
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
