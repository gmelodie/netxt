use crate::task::Task;

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
pub struct Section {
    pub name: String,
    pub tasks: Vec<Task>,
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
}
