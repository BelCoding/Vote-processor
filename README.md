# Project Name

## Description

A simplified tally functionality that processes incoming votes and outputs a result for a single-winner election using the first-past-the-post voting method.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Contributing](#contributing)

## Installation

To clone the project from GitHub, run the following command in your terminal:

```
git clone https://github.com/username/project.git
```

To build the project using Cargo, navigate to the project directory and run:

```
cargo build
```

## Usage

To run a command, use the following syntax:

```
cargo run -- <command>
```
Help:

```
cargo run -- --help
```

For example, to execute the programm using the files in the test-data folder, use:

```
cargo run -- --choices test-data/candidatures.json --votes test-data/voting_ballots.json --output-file test-data/results.json
```

## Testing

Run the following command to execute the tests:

```
cargo test
```

To see the debug messages as well, use the following command:

```
cargo test -- --show-output
```

## TODOs

Nice TODOs:
To define requirements for invalid input files, invalid choice ids or invalid lines in the ballots input file.
Based on these requirements implement the error handling, for example:
- Counting could be either interrupted or skip the errors just logging an error message.
- Errors could be propagated to the caller using ? operator and returning Result, to give more control to the caller.

Business logic details:
- The programm asserts with E_INVADID_CONTEST_ID message when a contest_id in votes_file is wrong.
- In case of draw the programm does not handle it, It ignores the second winner, it will depend on the position when hashing the map or in the implementation of the Counter.
- In case of invalid choice ID, the program prints an error to stdErr and the vote is mapped still with the non existing ID.

