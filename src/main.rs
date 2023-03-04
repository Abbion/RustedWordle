
use std::io::{BufReader, BufRead};
use std::io;
use std::fs::File;
use rand::Rng;

fn main() {
    println!("Welcome to rust wordle!");

    let random_word = load_random_word_from_list();

    let random_word = match random_word{
        Some(word) =>  word,
        None => panic!("No words found :("),
    };

    println!("Random wordle word {}", random_word);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("You entered {}", input);
}

fn load_random_word_from_list() -> Option<String> {
    let words_file = File::open("assets/words.txt");

    match words_file {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            let word_vec : Vec<String> = buf_reader.lines().map(|line| line.unwrap()).collect();
        
            if word_vec.len() == 0 {
                panic!("The file has no words!")
            }

            let random_line = rand::thread_rng().gen_range(0..word_vec.len());
            let random_word = word_vec.get(random_line).map(|s| s.to_owned());
        
            return random_word;
        },
        Err(error) => panic!("Can't open the file {:?}", error),
    };
}