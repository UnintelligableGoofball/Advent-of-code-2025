use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let rangs = parse_line("../d2_input");
}

fn parse_line(path: impl AsRef<Path>) {
    let file = File::open(path).expect("no fiilleeeeeeeeeeeeee wahhhhhh");
    let mut buf = BufReader::new(file);

    let mut ranges_line = String::new();
    buf.read_line(&mut ranges_line).expect("naur");
    ranges_line = ranges_line.trim().to_string();
    println!("{:?}", ranges_line);

    let ranges_iter = ranges_line.split(",");

    let rangs_vec: Vec<String> = ranges_iter.to_string().collect();
    
    
    }



