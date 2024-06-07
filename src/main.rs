mod args;
use args::Arguments;
use clap::Parser;

fn main() {
    let args = Arguments::parse();
    let choices_json = args.choices_json;
    let votes_json = args.votes_json;
    println!("{}", choices_json);
    println!("{}", votes_json);
}
