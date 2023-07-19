use clap::{Arg, Command};
use regex::Regex;

fn main() {
    let quote = "Every face, picture";

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
                        .help("string to search"),
                ),
        );

    let matches = cli.get_matches();
    let subcommands = matches.subcommand();

    match subcommands {
        Some(("pattern", pattern_matches)) => {
            if pattern_matches.contains_id("query") {
                let query = pattern_matches
                    .get_one::<String>("query")
                    .expect("contains_query");
                find_matches(query, quote);
            }
        }
        _ => unreachable!(),
    }
}

fn find_matches(query: &String, quote: &str) {
    let re = Regex::new(query).unwrap();
    for line in quote.lines() {
        let substr = re.find(line);
        match substr {
            Some(_) => println!("{}", line),
            None => (),
        };
    }
}