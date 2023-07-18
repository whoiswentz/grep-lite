use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();
    let quote = "Every face, picture";

    for line in quote.lines() {
        let substr = re.find(line);
        match substr {
            Some(_) => println!("{}", line),
            None => ()
        }
    }
}
