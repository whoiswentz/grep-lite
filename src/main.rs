use clap::{Arg, Command};
use regex::Regex;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    let cli = Command::new("grep-lite")
        .author("Vinicios Wentz")
        .about("searches for patterns")
        .version("0.01")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("pattern")
                .short_flag('p')
                .long_flag("pattern")
                .about("pattern commands")
                .arg(
                    Arg::new("query")
                        .short('q')
                        .long("query")
                        .required(true)
                        .help("The Pattern to search for"),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .required(true)
                        .help("File  to search"),
                ),
        );

    let matches = cli.get_matches();
    let subcommands = matches.subcommand();

    match subcommands {
        Some(("pattern", pattern_matches)) => {
            if pattern_matches.contains_id("query") {
                let query = pattern_matches
                    .get_one::<String>("query")
                    .expect("Query should be provided");
                let input = pattern_matches
                    .get_one::<String>("input")
                    .expect("Input should be provided");

                let file = File::open(input).expect("File does not exist");

                find_matches(query, file);
            }
        }
        _ => unreachable!(),
    }
}

fn find_matches(query: &str, file: File) {
    let re = Regex::new(query).unwrap();
    let reader = BufReader::new(file);
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        let substr = re.find(&line);
        if substr.is_some() {
            println!("{}", line)
        }
    }
}
