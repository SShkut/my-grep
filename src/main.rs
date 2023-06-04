use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    search_pattern: String,
    file_name: String,
}

fn main() {
    let args = Args::parse();

    let search_term = Regex::new(&args.search_pattern).unwrap();
    let file = File::open(&args.file_name);

    let text = match file {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Cannot open a file: {}\\n{}", &args.file_name, e),
    };

    for (line_number, line) in text.lines().enumerate() {
        let line = match line {
            Ok(result) => result,
            Err(_) => String::from(""),
        };

        let contains_pattern = search_term.find(&line);

        match contains_pattern {
            Some(_) => println!("{}, {}", line_number, line),
            None => (),
        }
    }
}
