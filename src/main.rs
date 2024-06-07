mod args;
use args::Arguments;
use clap::Parser;
mod contest;
use contest::Contest;

fn main() {
    let args = Arguments::parse();
    let choices_json = args.choices_json;
    let votes_json = args.votes_json;
    println!("{}", choices_json);

    let contest = Contest::new(&choices_json);
    dbg!(contest);

    println!("{}", votes_json);
    // TODO:
    // contest.count_ballots(votes_json);    
}
