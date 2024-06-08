mod args;
use args::Arguments;
use clap::Parser;
mod contest;
use contest::Contest;
use contest::ContestResult;

fn main() {
    let args = Arguments::parse();
    dbg!(&args);
    let contest = Contest::new(&args.choices_json);
    // Print contest id and contest description
    println!("Contest ID: {}", contest.get_contest_id());
    println!("Contest Description: {}", contest.get_description());

    let res = ContestResult::new(contest, &args.votes_json);

    match res.save_results_json(&args.output_json) {
        Ok(()) => println!("Results saved successfully"),
        Err(err) => println!("Error saving results: {}", err),
    }
}
