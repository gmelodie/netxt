use crate::section::{Section, SectionIterator};
use crate::util::*;
use chrono::NaiveDate;
use itertools::Itertools;
use std::cmp::Ordering;
use std::error;
use std::fmt;
use std::str;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Day {
    pub date: NaiveDate,
    pub sections: Vec<Section>,
}
impl Day {
    pub fn new(date: NaiveDate) -> Day {
        let sections: Vec<Section> = Vec::new();
        Day { date, sections }
    }
}

// when comparing with greater, compare dates
impl Ord for Day {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}
impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl str::FromStr for Day {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let text = s.trim().to_string();
        // first line must be the date
        let mut lines = text.lines();
        let date = NaiveDate::parse_from_str(lines.next().unwrap().trim(), "[%Y-%m-%d]")?;

        let mut sections: Vec<Section> = Vec::new();
        // the rest of the lines should be sections with tasks
        let section_iter = SectionIterator::new(&text);
        for section in section_iter {
            sections.push(section);
        }
        Ok(Day { date, sections })
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date = self.date.format("[%Y-%m-%d]").to_string();
        let sections = self.sections.iter().join("\n\n");
        write!(f, "{date}\n{sections}")
    }
}

pub struct DayIterator<'a> {
    lines: Vec<&'a str>,
    index: usize,
}

impl<'a> DayIterator<'a> {
    pub fn new(s: &'a str) -> Self {
        DayIterator {
            lines: s.lines().collect(),
            index: 0,
        }
    }
}

impl<'a> Iterator for DayIterator<'a> {
    type Item = Day;

    fn next(&mut self) -> Option<Self::Item> {
        let mut day: Vec<&'a str> = Vec::new();
        if self.index >= self.lines.len() {
            None
        } else {
            // first line must be start of a day, consume anything that is not start of day
            while self.index < self.lines.len() && !is_day_start(self.lines[self.index]) {
                self.index += 1;
            }
            if self.index >= self.lines.len() {
                return None;
            }

            // read start of day
            let start = self.lines[self.index];
            day.push(start);
            self.index += 1;

            // read everything until start of next day
            while self.index < self.lines.len() && !is_day_start(self.lines[self.index]) {
                day.push(self.lines[self.index]);
                self.index += 1;
            }

            let day: Day = day.join("\n").parse().expect("Unable to parse day");
            Some(day)
        }
    }
}

fn is_day_start(line: &str) -> bool {
    let line = line.trim();
    if line.is_empty() {
        return false;
    }
    let first_char = line.chars().next();
    if let Some(c) = first_char {
        if c == '[' {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::task::Task;
    use indoc::indoc;

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

        let actual: Day = day_text.parse().expect("Unable to parse day");
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

    #[test]
    fn day_with_anonymous_section() {
        let day_text = indoc! {"
            [2024-03-06]
            - task A
            - task B
            - task C

            Section 2
            - task 2.1
            - task 2.2
            Section 3
            - task 3.2
            - task 3.1
        "};

        let actual: Day = day_text.parse().expect("Unable to parse day");
        let expected = Day {
            date: NaiveDate::from_ymd_opt(2024, 3, 6).unwrap(),
            sections: vec![
                Section {
                    name: "".to_string(),
                    tasks: vec![
                        Task {
                            text: "task A".to_string(),
                        },
                        Task {
                            text: "task B".to_string(),
                        },
                        Task {
                            text: "task C".to_string(),
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
