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
    let mut ranges_line_str = ranges_line.trim();
    println!("{:?}", ranges_line_str);

    struct Elf_Range {
        
    }

    }



