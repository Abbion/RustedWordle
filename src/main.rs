
use std::io::{BufReader, BufRead};
use std::io::{self, Write};
use std::fs::File; 
use rand::Rng;
use colored::*;
use std::thread::sleep;
use std::time::Duration;
use console::Term;

#[derive(Debug)]
#[derive(PartialEq)]
enum LetterState {
    NoHit,
    Hit,
    Fit,
}

fn main() {

    loop {
        let random_word = match load_random_word_from_list() {
            Some(word) => word,
            None => panic!("No random word found!"),
        };

        new_round_loop(&random_word);
        
        if should_start_next_round() == false {
            break;
        }
    }

    println!("Thanks for playing :)");
}

fn should_start_next_round() -> bool {
    println!("Press 1 to start new wordle or press any other key to exit.");

    let term = Term::stdout();
    match term.read_char() {
        Ok(character) => { 
            if character == '1'{
                return true;
            }
            else {
                return false;
            }
        },
        _ => { return false; }
    };
}

fn new_round_loop(random_word : &String) {
    let mut user_guesses : Vec<String> = Vec::new();

    loop {
        print_interface(&user_guesses, &random_word);

        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("Failed to read input");
        user_guess = user_guess.trim().to_uppercase();

        if check_user_input(&user_guess, &user_guesses) {
            let letter_state = check_user_word(&user_guess, &random_word);
            user_guesses.push(user_guess);

            if chack_full_hit(&letter_state) {
                print_interface(&user_guesses, &random_word);
                println!("\nYou won!");
                break;
            }

            if user_guesses.len() >= 5 {
                print_interface(&user_guesses, &random_word);
                println!("\nThe word was: {}", random_word);
                break;
            }
        }
        else {
            sleep(Duration::from_secs(2));
            continue;
        }
    }
}

fn chack_full_hit(letter_state: &Vec<LetterState>) -> bool {
    return !(letter_state.contains(&LetterState::NoHit) || letter_state.contains(&LetterState::Fit));
}

fn check_user_input(user_guess : &String, user_guesses: &Vec<String>) -> bool{
    if !string_contains_only_letters(user_guess) {
        println!("{}", "Word should contain only letters!".yellow());
        return false;
    }

    if user_guess.len() > 5 {
        println!("{}", "Word is to long! It should have 5 letters.".yellow());
        return false;
    }

    if user_guess.len() < 5 {
        println!("{}", "Word is to short! It should have 5 letters.".yellow());
        return false;
    }

    if user_guesses.contains(user_guess) {
        println!("{}", "You guessed this word already!".yellow());
        return false;
    }

    return true;
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn string_contains_only_letters(string : &String) -> bool {
    for character in string.chars() {
        if !character.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn print_interface(user_guesses: &Vec<String>, random_word: &String) {
    clear_console();
    println!("Welcome to rust wordle!");

     println!("Random wordle word {}", random_word);

     for i in 0..5 {
         print!("{}: ", i + 1);
         match user_guesses.get(i) {
             Some(user_guess) => {
                 let letter_state = check_user_word(user_guess, random_word);
                 print_user_guess(user_guess, &letter_state);
             },
             None => { println!("___ ___ ___ ___ ___") },
         };
     }

    print!("Your guess: ");
    io::stdout().flush().unwrap();
}

fn print_user_guess(user_str: &String, letter_state: &Vec<LetterState>) {
    for (i, letter) in user_str.chars().enumerate() {
        let state = match letter_state.get(i){
            Some(state) => state,
            None => &LetterState::NoHit,
        };

        let letter_with_spacing = format!(" {} ", letter);

        match state {
            LetterState::NoHit => print!("{} ", letter_with_spacing.on_truecolor(109, 109, 109)),
            LetterState::Hit => print!("{} ", letter_with_spacing.on_truecolor(83, 141, 78)),
            LetterState::Fit => print!("{} ", letter_with_spacing.on_truecolor(231, 184, 52)),
        }
    }
    println!();
}

fn check_user_word(user_str: &String, word_str: &String) -> Vec<LetterState> {
    let mut letter_states = vec![LetterState::NoHit, LetterState::NoHit, LetterState::NoHit, LetterState::NoHit, LetterState::NoHit];
    let mut checked_letters_vec = word_str.as_bytes().to_vec();

    //Check for hit(higher priority)
    for (i, &letter_u) in user_str.as_bytes().iter().enumerate() {
        for (j, _) in word_str.as_bytes().iter().enumerate() {
            if letter_u != checked_letters_vec[j] {
                continue;
            }
            else if i == j {
                letter_states[i] = LetterState::Hit;
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
                letter_states[i] = LetterState::Fit;
                checked_letters_vec[j] = b'+';
            }
        }
    }

    return  letter_states;
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
            let random_word = word_vec.get(random_line).map(|s| s.to_uppercase().to_owned());
        
            return random_word;
        },
        Err(error) => panic!("Can't open the file {:?}", error),
    };
}