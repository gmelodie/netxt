use crate::task::Task;
use crate::util::Result;

use itertools::Itertools;
use std::error;
use std::fmt;
use std::str;

pub struct SectionIterator<'a> {
    lines: Vec<&'a str>,
    index: usize,
}

impl<'a> SectionIterator<'a> {
    pub fn new(s: &'a str) -> Self {
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
            while self.index < self.lines.len() && !is_section_end(self.lines[self.index]) {
                section.push(self.lines[self.index]);
                self.index += 1;
            }

            let section = section
                .join("\n")
                .parse()
                .expect("Unable to parse section text");

            Some(section)
        }
    }
}

fn is_section_start(line: &str) -> bool {
    if line.trim().is_empty() {
        return false;
    }
    let first_char = line.chars().next();
    match first_char {
        Some('-') => true, // tasks start anonymous sections
        Some('[') => false,
        None => false,
        _ => true,
    }
}

fn is_section_end(line: &str) -> bool {
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Section {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Section {
    pub fn new(name: &str) -> Section {
        Section {
            name: name.to_string(),
            tasks: Vec::<Task>::new(),
        }
    }
}

impl str::FromStr for Section {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let text = s.trim().to_string();

        // first line may be the section name
        // if first line is task, it means it's an anonymous section
        let mut lines = text.lines();
        let first_line = lines
            .next()
            .expect("Unable to read section name")
            .trim()
            .to_string();

        let mut tasks: Vec<Task> = Vec::new();

        let first_char = first_line.chars().next();
        let name = match first_char {
            Some('-') => {
                // tasks start anonymous sections
                // if first line is a task this means this section is anonymous
                tasks.push(first_line.parse().expect("Unable to parse task"));
                "".to_string()
            }
            _ => first_line,
        };

        // the rest of the lines should be tasks
        for line in lines {
            tasks.push(line.parse().expect("Unable to parse task"));
        }
        Ok(Section { name, tasks })
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.clone();
        let tasks = self.tasks.iter().join("\n");
        write!(f, "{name}\n{tasks}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_section() {
        let section_text = indoc! {"
            Some Section
            - task 1
            - task 3
            - task 2

        "};

        let actual: Section = section_text.parse().expect("Unable to parse day");
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
}
