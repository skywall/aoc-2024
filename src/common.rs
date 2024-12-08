use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn load_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    Ok(lines)
}
