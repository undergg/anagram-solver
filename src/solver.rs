use super::trie::Trie;
use std::{collections::{HashMap, HashSet}, todo};

pub trait AnagramSolver<'a> {
    fn find_all_anagrams(&self, ch: &str) -> HashSet<String>;
    fn add_dictionary(&mut self, dictionary : &'a Vec<&str>);
}

// Implement the AnagramSolver for the HashSet. (Explicit for str).
impl<'a> AnagramSolver<'a> for HashSet<&'a str> {
    fn find_all_anagrams(&self, ch: &str) -> HashSet<String> {
        let mut anagrams: HashSet<String> = HashSet::new();

        // Start with the empty string.
        let mut combination = String::new();

        // HashMap with all indices of input.
        let mut used: HashMap<usize, bool> = HashMap::new();

        for i in 0..ch.len() {
            used.insert(i, false);
        }

        find_all_anagrams_recurse(&self, &mut combination, &ch, &mut used, &mut anagrams);

        anagrams
    }

    fn add_dictionary(&mut self, dictionary : &'a Vec<&str>) {
        for word in dictionary {
            self.insert(word);
        }
    }

}

// dict -> the current dictionary.
// char_combination -> current string combination we are trying.
// ch -> the characters we can use in our combinations.
// used -> which characters are used and which are currently up for grabs.
// anagrams -> the vector where we collect all the anagrams.
fn find_all_anagrams_recurse(
    dictionary: &HashSet<&str>,
    char_combination: &mut String,
    ch: &str,
    used: &mut HashMap<usize, bool>,
    anagrams: &mut HashSet<String>,
) {
    // Check if combination exists in current dictionary.
    if dictionary.contains(char_combination.as_str()) && !anagrams.contains(char_combination) {
        // Here comes an interesting question..
        // Do we want to clone the char combination or simply copy the word reference from the dictionary
        // since we know that we store somewhere already the char_combination?
        anagrams.insert(char_combination.clone());
    }

    let characters: Vec<char> = ch.chars().collect();

    // Try adding one character.
    for i in 0..characters.len() {
        // If we haven't used this character yet then let's go ahead and do that.
        if !used[&i] {
            // Push character.
            char_combination.push(characters[i]);
            // Mark character as used.
            used.insert(i, true);
            find_all_anagrams_recurse(dictionary, char_combination, ch, used, anagrams);

            // Undo previous changes.
            used.insert(i, false);
            char_combination.pop();
        }
    }
}
