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

        //likely could have been done a looooot cleaner with some sort of euclidian-division based operation but i couldnt figure that out
        
        position = position + turn;

        if position < 0 && position - turn == 0 {
            position += 100;
            //println!{"weh"};
        }

        while position < 0 {
            count += 1;
            position += 100;
            //println!{"uppies"};
        };

        while position > 100 {
            count += 1;
            position += -100;
            //println!{"downsies"};
        }

        if position == 100 {
            position += -100;
        }

        if position == 0 {
            count += 1;
            //println!{"nothing. absolutely nothing at all. zero. zilch. nada."}
        }

        //println!("Turn: {turn}, Position: {position}, Count: {count}");
                
    }

    count
}

