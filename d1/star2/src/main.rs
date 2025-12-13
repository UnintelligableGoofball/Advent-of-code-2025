use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let turns = parse_file("../d1_input");
    let times_zero = process_turns(50, turns);
    println!("YEAHHHHHHHHH {times_zero}");
}

fn parse_file(path: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(path).expect("couldn't open file");
    let buf = BufReader::new(file);

    let stringy_turns: Vec<String> = buf.lines()
        .map(|l| l.expect("couldn't parse line"))
        .collect();

    let mut destringed = Vec::new();

    for turn in stringy_turns.iter() {
        let polarity = if turn.chars().nth(0) == Some('L') {
            -1
        } else {
            1
        };

        let intern: i32 = turn.trim().trim_matches(char::is_alphabetic).parse().expect("fuuuuuuuck");

        destringed.push(intern * polarity)
    }

    destringed
    
}


fn process_turns(initial: i32, turns: Vec<i32>) -> i32 {
    // modulo is x.rem_euclid(y)
    let mut position = initial;
    let mut count = 0;

    for turn in turns {
        // whyyyyyyyyyy
        position = (position + turn).rem_euclid(100);
        count += if position == 0 {
            1
        } else {
            (position + turn).div_euclid(100).abs()
        }

        
        
    }

    count
}

