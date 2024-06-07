extern crate clap;
use clap::Parser;

// Enter new line after each doc line

/// Processes incoming votes and outputs a result for a single-winner election using the first-past-the-post voting method.
///
/// Input and output files are in Json format.
/// Provide 2 arguments poinitng to the JSON files. One for the choices and another one for the votes. 
#[derive(Debug, Parser)]
#[clap(author("Beltran Rodriguez"), version("v1.0.0"))]
pub struct Arguments {

    #[arg(short = 'c', long = "choices", required = true)]
    pub choices_json: String,

    #[arg(short = 'v', long = "votes", required = true)]
    pub votes_json: String,
}
