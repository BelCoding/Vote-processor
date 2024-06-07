use serde_json;
use counter::Counter;

/// Contest, holds the ID, description and choices of the contest
///  to register the candidatures JSON file.
/// Every vote as input will contain the choice_id (of a candidate).
#[derive(Debug)]
pub struct Contest {
    contest_id: u64,
    description: String,
    choices: Counter<u32, String>
}

impl Contest {

    /// Reads the candidatures JSON file and returns a Contest Object.
    pub fn new(candidatures_file: &str) -> Self {

        let file = std::fs::read_to_string(candidatures_file).expect("Could not read the file.");
        let choices: serde_json::Value = serde_json::from_str(file.as_str()).expect("Invalid JSON");
        dbg!(&choices);
        
        Contest{
            contest_id: choices["id"].as_u64().expect("Contest id could not be parsed"),
            description: choices["description"].to_string(),
            // TODO: Deserialize de Value::array into a Counter
            choices: Counter::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_choices() {
        // let input_file = "/home/calitanuc/Projects/poll/candidatures.json";
        let input_file = "./test-data/candidatures.json";
        let choices = Contest::new(input_file);
        assert_eq!(choices.contest_id, 1);
        assert_eq!(choices.description, "\"Best Programming Language\"");
    }
}