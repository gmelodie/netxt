use crate::err;
use crate::util::Result;
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Task {
    pub text: String,
}

impl FromStr for Task {
    type Err = Box<dyn error::Error + Send + Sync>;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();

        let first_char = s.chars().next();
        match first_char {
            Some('-') => {}                                // this is a task
            _ => return err!("Unable to parse task: {s}"), // this is not a task
        }

        Ok(Task {
            text: s.trim_start_matches(&['-', ' ', '\t']).to_string(), // trim - and spaces
        })
    }
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.text.clone();
        write!(f, "- {text}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_task() {
        let task_line = "- pick up dry cleaning";
        let actual: Task = task_line.parse().expect("Unable to parse task");
        let expected = Task {
            text: "pick up dry cleaning".to_string(),
        };
        assert_eq!(actual, expected);
    }
}
