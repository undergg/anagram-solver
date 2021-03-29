use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

use crate::solver::AnagramSolver;

// Implementation of Trie in rust. (aka Prefix tree)
pub struct Trie {
    root: TrieNode,
    size: i32,
}

// For simplicity let's start with char as our way to navigate the trie.
// TODO: We can make it generic and conditionally generic on traits that
// are iterable. (e.g String, Vec etc.)
struct TrieNode {
    // If this is true then
    value: bool,
    edges: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            value: false,
            edges: HashMap::new(),
        }
    }
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode {
                // We want root to be false, since it's an artificial node.
                value: false,
                edges: HashMap::new(),
            },
            size: 0,
        }
    }

    pub fn insert(&mut self, item: &str) {
        insert_helper(&mut self.root, &mut item.chars());

        // Increase trie's size.
        self.size += 1;
    }

    // We could return bool here, or Result. Let's revisit.
    pub fn delete(&mut self, item: &str) {
        let mut delete_node = true;
        if delete_helper(&mut self.root, &mut item.chars(), &mut delete_node) {
            self.size -= 1;
        }
    }

    pub fn contains(&self, item: &str) -> bool {
        contains_helper(&self.root, &mut item.chars())
    }
}

// Helper for Trie insert.
// node is the current node we are visiting in the tree.
fn insert_helper(node: &mut TrieNode, c_iter: &mut Chars) {
    // Check if we are the end of the string.
    let ch = match c_iter.next() {
        Some(c) => c,
        None => {
            // We are at the end of the word.
            node.value = true;
            return;
        }
    };

    // Create or get next node.
    // We need a mutable reference because we may change it in a later call in the recursion.
    let next_node: &mut TrieNode = match node.edges.get_mut(&ch) {
        Some(n) => n,
        None => {
            let new_node = TrieNode::new();
            node.edges.insert(ch, new_node);
            // We just added the value it's fine.
            node.edges.get_mut(&ch).expect("We just added the value..")
        }
    };

    insert_helper(next_node, c_iter);
}

// Delete helper for deleting a trie node.
// node -> current node we are at
// c_iter -> current character
// delete_node -> whether we also delete the nodes as we go.
// Set it to true if you want to delete nodes when removing a word. (Recommended)
// Returns true is the word was deleted, false if the word didn't exist and thus not deleted.
fn delete_helper(node: &mut TrieNode, c_iter: &mut Chars, delete_node: &mut bool) -> bool {
    let ch: Option<char>;
    let ret: bool;

    // Traverse the Trie, reach the end.
    match c_iter.next() {
        Some(c) => {
            // We will need it after traversing.
            ch = Some(c);
            // Go to the next node.
            let next_node: &mut TrieNode = match node.edges.get_mut(&c) {
                Some(n) => n,
                None => {
                    // We could return an error. For the moment let's prevent deleting
                    // any nodes by mistake.
                    *delete_node = false;
                    return false;
                }
            };

            ret = delete_helper(next_node, c_iter, delete_node)
        }
        None => {
            // We are at the end of the word.
            node.value = false;
            // The word is found. Let's propagate true upwards.
            ret = true;
            ch = None;
        }
    };

    // The reasoning here is simple.
    // As we go up the tree, if there even one node that has more than 1 children
    // we stop deleting the nodes.
    if node.edges.len() > 1 {
        *delete_node = false;
    }

    //
    if *delete_node {
        match ch {
            Some(c) => {
                node.edges.remove(&c);
            }
            // We are at the last character, do nothing.
            None => {}
        }
    }

    ret
}

fn contains_helper(node: &TrieNode, c_iter: &mut Chars) -> bool {
    let ch = match c_iter.next() {
        Some(c) => c,
        None => return true,
    };

    // Create or get next node.
    // We need a mutable reference because we may change it in a later call in the recursion.
    let next_node: &TrieNode = match node.edges.get(&ch) {
        Some(n) => n,
        None => return false,
    };

    return contains_helper(next_node, c_iter);
}

impl AnagramSolver for Trie {
    fn find_all_anagrams(&self, ch: &str) -> HashSet<String> {
        let mut curr: String = String::from("");
        let mut anagrams: HashSet<String> = HashSet::new();

        // HashMap with all occurrences per character. 
        let mut oc: HashMap<char, i32> = HashMap::new();

        for c in ch.chars() {
            if !oc.contains_key(&c) {
                oc.insert(c, 1);
            }
            else {
                oc.insert(c, oc[&c] + 1);
            }
        }

        find_all_anagrams_helper(&self.root, &mut curr, ch, &mut oc, &mut anagrams);
        anagrams
    }
}

// node -> the current trie node we are visiting.
// curr -> the current string represented by the current node. (Also the one we hopefully add to the list.)
// ch -> the set of characters we want to find anagrams for
// oc -> how many times can we take character c?
// anagrams -> the list of anagrams we construct as we go down the tree.
fn find_all_anagrams_helper(
    node: &TrieNode,
    curr: &mut String,
    ch: &str,
    oc: &mut HashMap<char, i32>,
    anagrams: &mut HashSet<String>,
) {
    println!("We are at current word: {}",curr);
    
    // We found an anagram. 
    if node.value {
        anagrams.insert(curr.clone());
    }

    // Iterate through the tree.
    for (c, next_node) in node.edges.iter() {

        // If we haven't used this character yet then let's go ahead and do that.
        if oc.contains_key(c) && oc[c] >= 1 {
            // Push character.
            curr.push(*c);
            // Reduce available characters c by 1. 
            oc.insert(*c, oc[c] - 1);

            find_all_anagrams_helper(&next_node, curr, ch, oc, anagrams);

            // Undo previous changes.
            oc.insert(*c, oc[c] + 1);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_add_contains() {
        let mut trie: Trie = Trie::new();

        trie.insert("item");
        trie.insert("meti");

        assert_eq!(trie.size, 2);
        assert_eq!(trie.contains("item"), true);
        assert_eq!(trie.contains("meti"), true);
        assert_eq!(trie.contains("item2"), false);
    }

    #[test]
    fn test_trie_delete() {
        let mut trie: Trie = Trie::new();

        trie.insert("item");
        trie.insert("meti");

        assert_eq!(trie.size, 2);

        trie.delete("item");

        assert_eq!(trie.size, 1);
    }

    #[test]
    fn test_trie_delete_word_does_not_exist() {
        let mut trie: Trie = Trie::new();

        trie.insert("item");
        trie.insert("meti");

        trie.delete("word");

        assert_eq!(trie.size, 2);
    }

    #[test]
    fn test_trie_find_anagrams() {
        let mut trie : Trie = Trie::new();

        trie.insert("anagram");
        trie.insert("one");
        trie.insert("two");
        trie.insert("ten");

        let anagrams = trie.find_all_anagrams("otwen");

        assert_eq!(anagrams.len(), 3);
        assert_eq!(anagrams.contains("one"), true);
        assert_eq!(anagrams.contains("two"), true);
        assert_eq!(anagrams.contains("ten"), true);
        assert_eq!(anagrams.contains("anagram"), false);

        let anagrams = trie.find_all_anagrams("");

        assert_eq!(anagrams.len(), 0);
    }
}
