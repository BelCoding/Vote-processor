use counter::Counter;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{collections::HashMap, fs::File};
use tracing::{debug, error, info};

// Errors
const E_INVALID_JSON: &str = "Invalid JSON format.";
const E_INVADID_CONTEST_ID: &str = "Invalid contest ID.";
const E_INVADID_BALLOT: &str = "Invalid ballot's choice ID.";
const E_READ: &str = "Could not read the file.";
const E_CREATE_FILE: &str = "Cannot not create the file.";
const E_SERIALIZE: &str = "Error to serialize.";
const E_DESERIALIZE: &str = "Error to deserialize.";
const E_WINNER_DATA: &str = "Error getting the Winner data.";

/// Mimics the content in the Jsonfil for each choice.
#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    id: u64,
    text: String,
}

/// Each line in the input file must represent a Ballot like this
#[derive(Debug, Deserialize)]
struct Ballot {
    contest_id: u64,
    choice_id: u64,
}

/// Contest, holds the ID, description and choices of the contest
///  to register the candidatures JSON file.
#[derive(Debug)]
pub struct Contest {
    contest_id: u64,
    description: String,
    choices: HashMap<u64, String>,
}

impl Contest {
    /// Deserialze the candidatures JSON file and returns a Contest Object.
    #[tracing::instrument(skip(candidatures_file))]
    pub fn new(candidatures_file: &str) -> Self {
        debug!(
            candidates_file = candidatures_file,
            "Reading candidatures file:"
        );
        let file = std::fs::read_to_string(candidatures_file).expect(E_READ);
        let contest: serde_json::Value = serde_json::from_str(file.as_str()).expect(E_INVALID_JSON);
        info!("Deserialized candidatures file.");
        debug!(contest = ?contest, "Deserialized contest value:");

        Contest {
            contest_id: contest["id"].as_u64().expect(E_INVALID_JSON),
            description: contest["description"].to_string(),
            choices: Self::deserialize_choices_array(contest["choices"].clone()),
        }
    }

    #[tracing::instrument(skip(array))]
    /// Parses the Json Value that points to the array of candidates.
    fn deserialize_choices_array(array: Value) -> HashMap<u64, String> {
        debug!(target: "choice", array = ?array, "Deserializing choices array:");

        let choices: Vec<Choice> = serde_json::from_value(array).expect(E_DESERIALIZE);
        let choices_map: HashMap<u64, String> = choices
            .into_iter()
            .map(|choice| (choice.id, choice.text))
            .collect();

        choices_map
    }

    // Getter functions for contest id, contest description and choices
    pub fn get_contest_id(&self) -> u64 {
        self.contest_id
    }

    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }
}

/// Mimics the content of one choice in the contest result (output file)
#[derive(Debug, Serialize)]
struct ChoiceResult {
    choice_id: u64,
    total_count: u64,
}

/// The whole contest result output JSON file
#[derive(Debug, Serialize)]
pub struct ContestResult {
    contest_id: u64,
    total_votes: u64,
    results: Vec<ChoiceResult>,
    winner: Choice,
}

/// Implement serialize to JSON for ContestResult with all fields set to dummy values
impl ContestResult {
    /// Creates a ContestResult object and computes the results on it.
    /// The first arg is the Contest obj that you must have created first.
    ///
    /// The input_file must contain the ballots in JSON format.
    /// Every vote as input will contain the context_id and the choice_id (of a candidate).
    ///
    /// Check save_results_json to serialize the results into a file.
    #[tracing::instrument(skip(contest, input_file))]
    pub fn new(contest: Contest, input_file: &str) -> Self {
        let mut winner_id = 0;
        let mut winner_text = "".to_string();
        let (results, total_votes) = Self::count_ballots(&contest, input_file);

        if total_votes > 0 {
            winner_id = results.first().expect(E_WINNER_DATA).choice_id;
            winner_text = contest
                .choices
                .get(&winner_id)
                .expect(E_WINNER_DATA)
                .to_string();
        }

        ContestResult {
            contest_id: contest.contest_id,
            total_votes,
            results,
            winner: Choice {
                id: winner_id,
                text: winner_text,
            },
        }
    }

    /// Returns a pair with:
    /// - An ordered vector(most voted first) with the results ready to be serialized.
    /// - The count of total votes.
    #[tracing::instrument(skip(contest, input_file))]
    fn count_ballots(contest: &Contest, input_file: &str) -> (Vec<ChoiceResult>, u64) {
        let mut map: HashMap<u64, u64> = HashMap::new(); // meaning HashMap<choice_id, count>
        let f = File::open(input_file).expect(E_INVALID_JSON);
        let mut reader = BufReader::new(f);

        let mut total_votes: u64 = 0;
        let mut line = String::new();
        // Print each line in a loop until the end of the file is reached
        while reader.read_line(&mut line).is_ok_and(|bytes| bytes > 0) {
            if !line.trim().is_empty() {
                debug!(target: "ballot", line = ?line, "New ballot's line read:");
                let ballot: Ballot = serde_json::from_str(line.trim()).expect(E_INVALID_JSON);
                debug!(target: "ballot", ballot = ?ballot, "New ballot deserialized:");
                assert!(
                    ballot.contest_id == contest.contest_id,
                    "{}",
                    E_INVADID_CONTEST_ID
                );
                if !contest.choices.contains_key(&ballot.choice_id) {
                    error!("{}", E_INVADID_BALLOT);
                }

                // insert in map ballot.choice_id key with value 1, but if it exists update the value +1
                map.entry(ballot.choice_id)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                total_votes += 1;
            }

            line.clear();
        }

        info!(total_votes, "Total votes read:");
        // Order the map, the most common at the beginning
        let ordered_vector = map.into_iter().collect::<Counter<u64, u64>>().most_common();
        debug!(ordered_vector = ?ordered_vector, "Ordered vector:");

        let mut results: Vec<ChoiceResult> = vec![];
        // Iterate over ordered_vector and save each pair element into results vector
        for (key, value) in ordered_vector {
            results.push(ChoiceResult {
                choice_id: key,
                total_count: value,
            });
        }

        (results, total_votes)
    }

    /// Creates a JSON file, serialize ContestResult
    /// and write the content into the file.
    pub fn save_results_json(&self, output_file: &str) -> std::io::Result<()> {
        let content: String = serde_json::to_string_pretty(&self).expect(E_SERIALIZE);
        let file = File::create(output_file).expect(E_CREATE_FILE);
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &format!("{}", content))?;
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid JSON format.")]
    fn test_empty_choices_file() {
        let input_file = "./test-data/candidatures_empty.json";
        let _contest = Contest::new(input_file);
    }

    #[test]
    fn test_basic_parse_choices_file() {
        // let input_file = "/home/calitanuc/Projects/poll/candidatures.json";
        let input_file = "./test-data/candidatures.json";
        let contest = Contest::new(input_file);
        assert_eq!(contest.contest_id, 1);
        assert_eq!(contest.description, "\"Best Programming Language\"");
        assert_eq!(contest.choices.len(), 3);
        assert_eq!(contest.choices.get(&1).unwrap(), "Rust");
    }

    #[test]
    fn test_empty_ballots_file() {
        let candidatures_file = "./test-data/candidatures.json";
        let votes_file = "./test-data/voting_ballots_empty.json";
        let contest = Contest::new(candidatures_file);
        let contest_result = ContestResult::new(contest, votes_file);
        assert_eq!(contest_result.total_votes, 0);
    }

    #[test]
    fn test_basic_contest_result() {
        let candidatures_file = "./test-data/candidatures.json";
        let votes_file = "./test-data/voting_ballots.json";
        let contest = Contest::new(candidatures_file);
        let contest_result = ContestResult::new(contest, votes_file);
        assert_eq!(contest_result.total_votes, 4);
        assert_eq!(contest_result.winner.text, "Rust");
        assert_eq!(contest_result.winner.id, 1);
    }

    // In case of draw the winner will depend on the position when hashing the map
    // The business logic in this case is not handled.
    #[test]
    fn test_basic_contest_result_draw() {
        let candidatures_file = "./test-data/candidatures.json";
        let votes_file = "./test-data/voting_ballots_draw.json";
        let contest = Contest::new(candidatures_file);
        let contest_result = ContestResult::new(contest, votes_file);
        assert_eq!(
            contest_result.results[0].total_count,
            contest_result.results[1].total_count
        );
    }

    #[test]
    fn test_basic_contest_result_rust_by_one() {
        let candidatures_file = "./test-data/candidatures.json";
        let votes_file = "./test-data/voting_ballots_RustbyOne.json";
        let contest = Contest::new(candidatures_file);
        let contest_result = ContestResult::new(contest, votes_file);
        assert_eq!(contest_result.winner.text, "Rust");
        assert_eq!(contest_result.winner.id, 1); // Rust won by one vote difference only
        assert_eq!(
            contest_result.results[0].total_count,
            contest_result.results[1].total_count + 1
        );
    }

    // Test the function asserts with E_INVADID_CONTEST_ID message when a contest_id in votes_file is wrong.
    #[test]
    #[should_panic(expected = "Invalid contest ID.")]
    fn test_invalid_ballot_contest_id() {
        let candidatures_file = "./test-data/candidatures.json";
        let votes_file = "./test-data/voting_ballots_wrong_contest_id.json";
        let contest = Contest::new(candidatures_file);
        let _ = ContestResult::new(contest, votes_file);
    }

    #[test]
    fn test_save_contest_result() {
        let candidatures_file = "./test-data/candidatures.json";
        let votes_file = "./test-data/voting_ballots.json";
        let contest = Contest::new(candidatures_file);
        let contest_result = ContestResult::new(contest, votes_file);
        let output_file = "./test-data/result.json";
        contest_result
            .save_results_json(output_file)
            .expect("Error to create the file.");
    }
}
