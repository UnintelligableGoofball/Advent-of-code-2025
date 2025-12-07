use std::io;

fn main() {
    let mut start = String::new();
    let mut turn = String::new();
    
    println!("Current value?");

    io::stdin()
        .read_line(&mut start)
        .expect("input failure");

    println!("turn ammount?");

    io::stdin()
        .read_line(&mut turn)
        .expect("input failure");

    let start: i32 = start.trim().parse().expect("nan dummy");
    let turn: i32 = turn.trim().parse().expect("nan dummy");

    let result = &start + &turn;

    println!("end is: {result}")
    
}
