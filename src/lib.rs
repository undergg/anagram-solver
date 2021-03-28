use std::{collections::HashSet, io::stdin};

use solver::AnagramSolver;
use trie::Trie;

mod solver;
mod trie;

// Expose Config to get it in main.rs
pub struct Config {
    filename: String,
    mode: Mode,
}

// Mode to determine the speed of our anagram solver.
enum Mode {
    Slow,
    Fast,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            filename: String::from(""),
            mode: Mode::Fast,
        }
    }
}

impl Config {
    // Let's clone the args for now. Otherwise we'd have to introduce lifetimes
    // to the struct. (Let's do this as an exercise..)
    pub fn new(args: &[String]) -> Config {
        let mut config = Config::default();

        match args.len() {
            // We could instead of panicking here to return a Result<Config, Error>.
            0 | 1 => panic!("You need to pass the dictionary filename!"),
            2 => {
                config.filename = args[1].clone();
            }
            3 => {
                config.filename = args[1].clone();

                match args[2].as_str() {
                    "slow" => {
                        config.mode = Mode::Slow;
                    }
                    "fast" => {
                        config.mode = Mode::Fast;
                    }
                    _ => {
                        println!(
                            "Invalid mode was passed. We ignore and continue with default value."
                        );
                    }
                }
            }
            _ => {
                println!("Ignore additional parameters.");
            }
        }

        config
    }
}

// Reads the contents of the dictionary and returns a list of all the words.
pub fn extract_words(file_contents: &str) -> HashSet<&str> {
    let mut words: HashSet<&str> = HashSet::new();

    for line in file_contents.lines() {
        let space_separated_words: Vec<&str> = line.split(' ').collect();
        for word in space_separated_words {
            words.insert(word);
        }
    }

    return words;
}

// Make main light. Move all functionality here.
pub fn run(config: Config) {
    // Read the filename. Extract all words.
    let file_contents =
        std::fs::read_to_string(config.filename).expect("Failed to read file contents");

    // Borrow file_contents to the extract_words function.
    let dictionary: HashSet<&str> = extract_words(&file_contents);

    loop {
        println!("Pass a set of characters that you want to find anagram words for..");
        
        let mut characters: String = String::new();

        stdin()
            .read_line(&mut characters)
            .expect("Failed to read string.");

        let anagrams = dictionary.find_all_anagrams(characters.as_str());

        match anagrams.len() {
            0 => println!("No anagrams found!"),
            _ => {
                for key in anagrams.iter() {
                    println!("Anagram found:{}", key);
                }
            }
        }
    }
}

// TODO: Write tests for extract words.
