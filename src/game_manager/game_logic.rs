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
        if letter_u == checked_letters_vec[i] {
            letter_states[i] = LetterState::Hit;
            checked_letters_vec[i] = b'+';
        }
    }

    //Check for fit
    for (i, &letter_u) in user_str.as_bytes().iter().enumerate() {
        for (j, _) in word_str.as_bytes().iter().enumerate() {
            if letter_u != checked_letters_vec[j] {
                continue;
            }
            else if i != j && letter_states[i] != LetterState::Hit {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_when_full_hit_tests_gets_only_hit_status(){
        assert_eq!(true, check_full_hit(&vec![LetterState::Hit, LetterState::Hit, LetterState::Hit, LetterState::Hit, LetterState::Hit]));
    }

    #[test]
    fn should_return_false_when_full_hit_tests_gets_fit(){
        assert_eq!(false, check_full_hit(&vec![LetterState::Hit, LetterState::Fit, LetterState::Hit, LetterState::Hit, LetterState::Fit]));
    }

    #[test]
    fn should_return_false_when_full_hit_tests_gets_no_hit(){
        assert_eq!(false, check_full_hit(&vec![LetterState::NoHit, LetterState::NoHit, LetterState::Hit, LetterState::Hit, LetterState::Hit]));
    }

    #[test]
    fn should_return_false_when_full_hit_tests_gets_no_hit_or_fit(){
        assert_eq!(false, check_full_hit(&vec![LetterState::NoHit, LetterState::NoHit, LetterState::Fit, LetterState::Hit, LetterState::Fit]));
    }

    #[test]
    fn should_return_false_if_user_input_is_too_short(){
        assert_eq!(false, check_user_input(&String::from("cat"), &vec![]));
    }

    #[test]
    fn should_return_false_if_user_input_is_too_long(){
        assert_eq!(false, check_user_input(&String::from("guitar"), &vec![]));
    }

    #[test]
    fn should_return_false_if_user_input_has_non_alphabetic_symbols(){
        assert_eq!(false, check_user_input(&String::from("delt@"), &vec![]));
    }

    #[test]
    fn should_return_false_if_user_input_repeats(){
        assert_eq!(false, check_user_input(&String::from("cloud"), &vec![String::from("trade"), String::from("cloud")]));
    }

    #[test]
    fn should_return_true_if_user_input_has_five_letters(){
        assert_eq!(true, check_user_input(&String::from("tilde"), &vec![]));
    }

    #[test]
    fn should_return_true_if_input_has_only_letters() {
        assert_eq!(true, string_contains_only_letters(&String::from("decrypted")));
    }

    #[test]
    fn should_return_false_if_input_has_non_alphabetic_symbols() {
        assert_eq!(false, string_contains_only_letters(&String::from("m1a#")));
    }

    #[test]
    fn fit_state_test_1() {
        let letter_state_fn = check_user_word(&String::from("table"), &String::from("cloud"));
        let letter_state = vec![LetterState::NoHit, LetterState::NoHit, LetterState::NoHit, LetterState::Fit, LetterState::NoHit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn fit_state_test_2() {
        let letter_state_fn = check_user_word(&String::from("delta"), &String::from("glove"));
        let letter_state = vec![LetterState::NoHit, LetterState::Fit, LetterState::Fit, LetterState::NoHit, LetterState::NoHit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn fit_state_test_3() {
        let letter_state_fn = check_user_word(&String::from("tilde"), &String::from("adept"));
        let letter_state = vec![LetterState::Fit, LetterState::NoHit, LetterState::NoHit, LetterState::Fit, LetterState::Fit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn hit_state_test_1() {
        let letter_state_fn = check_user_word(&String::from("ready"), &String::from("renew"));
        let letter_state = vec![LetterState::Hit, LetterState::Hit, LetterState::NoHit, LetterState::NoHit, LetterState::NoHit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn hit_state_test_2() {
        let letter_state_fn = check_user_word(&String::from("trick"), &String::from("thief"));
        let letter_state = vec![LetterState::Hit, LetterState::NoHit, LetterState::Hit, LetterState::NoHit, LetterState::NoHit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn hit_state_test_3() {
        let letter_state_fn = check_user_word(&String::from("drive"), &String::from("using"));
        let letter_state = vec![LetterState::NoHit, LetterState::NoHit, LetterState::Hit, LetterState::NoHit, LetterState::NoHit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn mix_state_test_1() {
        let letter_state_fn = check_user_word(&String::from("musky"), &String::from("saucy"));
        let letter_state = vec![LetterState::NoHit, LetterState::Fit, LetterState::Fit, LetterState::NoHit, LetterState::Hit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn mix_state_test_2() {
        let letter_state_fn = check_user_word(&String::from("query"), &String::from("pulse"));
        let letter_state = vec![LetterState::NoHit, LetterState::Hit, LetterState::Fit, LetterState::NoHit, LetterState::NoHit];
        assert_eq!(letter_state_fn, letter_state);
    }

    #[test]
    fn mix_state_test_3() {
        let letter_state_fn = check_user_word(&String::from("cargo"), &String::from("apron"));
        let letter_state = vec![LetterState::NoHit, LetterState::Fit, LetterState::Hit, LetterState::NoHit, LetterState::Fit];
        assert_eq!(letter_state_fn, letter_state);
    }
}