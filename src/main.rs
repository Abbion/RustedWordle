
use std::io;

fn main() {
    println!("Welcome to rust wordle!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("you entered {}", input);
}
