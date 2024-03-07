use crate::section::{Section, SectionIterator};
use crate::util::Result;
use chrono::NaiveDate;
use std::error;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Day {
    pub date: NaiveDate,
    pub sections: Vec<Section>,
}
impl Day {
    pub fn new(date: NaiveDate) -> Day {
        let mut sections: Vec<Section> = Vec::new();
        Day { date, sections }
    }
}

impl FromStr for Day {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let text = s.trim().to_string();
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
        Ok(Day { date, sections })
    }
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
}