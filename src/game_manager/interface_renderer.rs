use colored::*;
use std::io::{self, Write};
use super::game_logic::{clear_console, check_user_word, LetterState};

pub fn print_interface(user_guesses: &Vec<String>, random_word: &String) {
    clear_console();
    println!("Welcome to rust wordle!");

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

pub fn print_user_guess(user_str: &String, letter_state: &Vec<LetterState>) {
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


