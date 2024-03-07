#[derive(PartialEq, Debug)]
pub struct Task {
    pub text: String,
}

impl Task {
    pub fn new(text: &str) -> Task {
        Task {
            text: text.trim_start_matches(&['-', ' ']).to_string(), // trim - and spaces
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_task() {
        let task_line = "- pick up dry cleaning";
        let actual = Task::new(task_line);
        let expected = Task {
            text: "pick up dry cleaning".to_string(),
        };
        assert_eq!(actual, expected);
    }
}
