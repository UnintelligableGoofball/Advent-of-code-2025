use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    cmp::Eq
};

fn main() {
    let turns = parse_file("../d1_input");
    for turn in turns {
        println!("{:?}", turn);
    }
}

fn parse_file(path: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(path).expect("couldn't open file");
    let buf = BufReader::new(file);

    let stringy_turns: Vec<String> = buf.lines()
        .map(|l| l.expect("couldn't parse line"))
        .collect();

    let mut vec = Vec::new();

    for turn in stringy_turns.iter() {
        let polarity = if turn.chars().nth(0) == Some('L') {
            -1
        } else {
            1
        };

        let intern: i32 = turn.trim().trim_matches(char::is_alphabetic).parse().expect("fuuuuuuuck");

        vec.push(intern * polarity)
    }

    vec
    
}

/*
fn process_turns(initial: i32, turns: Vec<String>) -> i32 {
    // modulo is x.rem_euclid(y)
    
}
*/
