use std::io;
use std::thread::sleep;
use std::time::Duration;
use console::Term;

mod game_logic;
use game_logic::{check_user_input, check_user_word, check_full_hit};

mod interface_renderer;
use interface_renderer::*;

pub fn should_start_next_round() -> bool {
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

pub fn new_round_loop(random_word : &String) {
    let mut user_guesses : Vec<String> = Vec::new();

    loop {
        print_interface(&user_guesses, &random_word);

        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("Failed to read input");
        user_guess = user_guess.trim().to_uppercase();

        if check_user_input(&user_guess, &user_guesses) {
            let letter_state = check_user_word(&user_guess, &random_word);
            user_guesses.push(user_guess);

            if check_full_hit(&letter_state) {
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