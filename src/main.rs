use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    search_pattern: String,
}

fn main() {
    let args = Args::parse();

    let search_term = Regex::new(&args.search_pattern).unwrap();
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for (line_number, line) in quote.lines().enumerate() {
        let contains_pattern = search_term.find(line);

        match contains_pattern {
            Some(_) => println!("{}, {}", line_number, line),
            None => (),
        }
    }
}
