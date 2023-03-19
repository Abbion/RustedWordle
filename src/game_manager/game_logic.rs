use colored::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LetterState {
    NoHit,
    Hit,
    Fit,
}

pub fn check_full_hit(letter_state: &Vec<LetterState>) -> bool {
    return !(letter_state.contains(&LetterState::NoHit) || letter_state.contains(&LetterState::Fit));
}

pub fn check_user_input(user_guess : &String, user_guesses: &Vec<String>) -> bool{
    if !string_contains_only_letters(user_guess) {
        println!("{}", "Word should contain only letters!".yellow());
        return false;
    }

    if user_guess.len() > 5 {
        println!("{}", "Word is too long! It should have 5 letters.".yellow());
        return false;
    }

    if user_guess.len() < 5 {
        println!("{}", "Word is too short! It should have 5 letters.".yellow());
        return false;
    }

    if user_guesses.contains(user_guess) {
        println!("{}", "You guessed this word already!".yellow());
        return false;
    }

    return true;
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn check_user_word(user_str: &String, word_str: &String) -> Vec<LetterState> {
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

fn string_contains_only_letters(string : &String) -> bool {
    for character in string.chars() {
        if !character.is_alphabetic() {
            return false;
        }
    }
    return true;
}