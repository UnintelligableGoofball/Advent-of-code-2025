use std::io::{prelude::*, BufReader};
use std::fs;
use std::path;

fn main() {
    let instructions = parse_file("../d1_input");
    for instruction in instructions {
        println!("{:?}", instruction);
    }
    
}

fn parse_file(path: impl AsRef<path::Path>) -> Vec<String> {
    let file = fs::File::open(path).expect("couldn't open file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("couldn't parse line"))
        .collect()
}
