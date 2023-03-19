
use std::io::{BufReader, BufRead};
use std::fs::File; 
use rand::Rng;

pub fn load_random_word_from_list() -> String {
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
        
            return  match random_word {
                Some(word) => word,
                None => panic!("No random word found!"),
            };
        },
        Err(error) => panic!("Can't open the file {:?}", error),
    };
}