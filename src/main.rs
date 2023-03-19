mod game_manager;
use game_manager::{new_round_loop, should_start_next_round};

mod random_word_generator;
use random_word_generator::load_random_word_from_list;

fn main() {

    loop {
        let random_word = load_random_word_from_list();

        new_round_loop(&random_word);
        
        if should_start_next_round() == false {
            break;
        }
    }

    println!("Thanks for playing :)");
}