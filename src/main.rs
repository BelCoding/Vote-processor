mod args;
use args::Arguments;
use clap::Parser;
mod contest;
use contest::Contest;
use contest::ContestResult;
use tracing::span;
use tracing::Level;
use tracing::{debug, error, info};

fn main() {
    // Subscribe to the tracing subscriber
    tracing_subscriber::fmt::init();

    let args = Arguments::parse();
    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    debug!(args=?args);
    // Create the contest object with the available data from the file choices_json
    let contest = Contest::new(&args.choices_json);
    // Print contest id and contest description:
    info!("Contest ID: {}", contest.get_contest_id());
    info!("Contest Description: {}", contest.get_description());
    // Compute the results from the file votes_json
    let res = ContestResult::new(contest, &args.votes_json);

    match res.save_results_json(&args.output_json) {
        Ok(()) => info!("Results saved successfully"),
        Err(err) => error!("Error saving results: {}", err),
    }
}
