
use std::io::{BufReader, BufRead};
//use std::io;
use std::fs::File;
use rand::Rng;
use colored::*;

#[derive(Debug)]
enum LetterState {
    NoHit,
    Hit,
    Fit,
}

fn main() {
    println!("Welcome to rust wordle!");

    let random_word = load_random_word_from_list();

    let random_word = match random_word{
        Some(word) =>  word,
        None => panic!("No words found :("),
    };

    println!("Random wordle word {}", random_word);

    //let mut input = String::new();
    //io::stdin().read_line(&mut input).expect("Failed to read input");
    //println!("You entered {}", input);


    let str1 = String::from("recog");
    let str2 = String::from("regal");

    let letter_states = check_user_word(&str1, &str2);
    
    for state in &letter_states{
        println!("{:?}", state);
    }

    print_user_guess(&str1, &letter_states);

}

fn print_user_guess(user_str: &String, word_state: &Vec<LetterState>) {
    for (i, letter) in user_str.chars().enumerate() {
        
    }

    println!("This is green {}.", user_str.on_truecolor(83, 141, 78));
    println!("This is yello {}.", user_str.on_truecolor(231, 184, 52));
    println!("This is gray {}.", user_str.on_truecolor(109, 109, 109));
}

fn check_user_word(user_str: &String, word_str: &String) -> Vec<LetterState> {
    let mut word_state = vec![LetterState::NoHit, LetterState::NoHit, LetterState::NoHit, LetterState::NoHit, LetterState::NoHit];
    let mut checked_letters_vec = word_str.as_bytes().to_vec();

    //Check for hit(higher priority)
    for (i, &letter_u) in user_str.as_bytes().iter().enumerate() {
        for (j, _) in word_str.as_bytes().iter().enumerate() {
            if letter_u != checked_letters_vec[j] {
                continue;
            }
            else if i == j {
                word_state[i] = LetterState::Hit;
                checked_letters_vec[j] = b'+';
            }
        }
    }

    //Check for fit
    for (i, &letter_u) in user_str.as_bytes().iter().enumerate() {
        for (j, _) in word_str.as_bytes().iter().enumerate() {
            if letter_u != checked_letters_vec[j] {
                continue;
            }
            else if i != j {
                word_state[i] = LetterState::Fit;
                checked_letters_vec[j] = b'+';
            }
        }
    }

    return  word_state;
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