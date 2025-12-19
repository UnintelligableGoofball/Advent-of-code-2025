use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let input = parse_line("../d2_input");
    println!("{:?}", input);
    let yay = process_ranges(input);
}

fn parse_line(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("no fiilleeeeeeeeeeeeee wahhhhhh");
    let mut buf = BufReader::new(file);

    let mut ranges_line = String::new();
    buf.read_line(&mut ranges_line).expect("naur");
    ranges_line = ranges_line.trim().to_string();

    let rangs: Vec<String> = ranges_line.split(",").map(|s| s.to_string()).collect();
    
    rangs
    
}

fn process_ranges(ranges: Vec<String>) -> i32 {
    for range in ranges {
        let ends: Vec<String> = range.split("-").map(|s| s.to_string()).collect();
        for i in ends[1].parse::<i32>().unwrap()..ends[2].parse::<i32>().unwrap() {
            if (i / 2) == i.to_string().split_at(i.to_string().len()).0.parse::<i32>().unwrap() {
                println!("yay");
            }
        }
    }
    1
}
