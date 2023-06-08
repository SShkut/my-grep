use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn find_in_file(file_name: &str, search_pattern: &str) {
    let file = File::open(file_name);

    let text = match file {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Cannot open a file: {}\\n{}", file_name, e),
    };

    process_lines(text, &search_pattern)
}

pub fn find_in_std_in(search_pattern: &str) {
    let stdin = io::stdin();
    let reader = stdin.lock();

    process_lines(reader, &search_pattern)
}

pub fn process_lines<T: BufRead + Sized>(reader: T, search_pattern: &str) {
    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(result) => result,
            Err(_) => String::from(""),
        };
        let contains_pattern = line.contains(&search_pattern);
        if contains_pattern {
            println!("{}: {}", line_number + 1, line);
        }
    }
}
