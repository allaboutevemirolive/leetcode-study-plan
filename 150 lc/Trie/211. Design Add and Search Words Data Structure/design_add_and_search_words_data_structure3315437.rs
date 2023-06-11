// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3315437/rust-trie-memoization-with-hashmap-and-wildcards/
use std::collections::HashMap;

// Define TrieNode structure
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_word: false,
        }
    }
}

// Define WordDictionary structure
struct WordDictionary {
    root: TrieNode,
    words: Vec<String>,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: TrieNode::new(),
            words: Vec::new(),
        }
    }

    // Add a word to the dictionary
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[index].as_deref_mut().unwrap();
        }
        node.is_word = true;
        self.words.push(word);
    }

    // Search for a word in the dictionary
    fn search(&self, word: String) -> bool {
        let wildcard_count = word.chars().filter(|&c| c == '.').count();
        if wildcard_count as f64 / word.len() as f64 > 0.66 {
            for stored_word in self.words.iter() {
                if Self::match_words(stored_word, &word) {
                    return true;
                }
            }
            false
        } else {
            let mut memo = HashMap::new();
            Self::search_helper(&word, &self.root, 0, &mut memo)
        }
    }

    // Helper function for searching
    fn search_helper(word: &str, node: &TrieNode, pos: usize, memo: &mut HashMap<(usize, *const TrieNode), bool>) -> bool {
        if pos == word.len() {
            return node.is_word;
        }
        
        if let Some(&result) = memo.get(&(pos, node as *const TrieNode)) {
            return result;
        }

        let c = word.chars().nth(pos).unwrap();
        let mut result = false;
        if c == '.' {
            for child in node.children.iter() {
                if let Some(ref child_node) = child {
                    if Self::search_helper(word, child_node, pos + 1, memo) {
                        result = true;
                        break;
                    }
                }
            }
        } else {
            let index = (c as u8 - b'a') as usize;
            if let Some(ref child_node) = node.children[index] {
                result = Self::search_helper(word, child_node, pos + 1, memo);
            }
        }

        memo.insert((pos, node as *const TrieNode), result);
        result
    }

    // Check if two words match with wildcard support
    fn match_words(word1: &str, word2: &str) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        for (c1, c2) in word1.chars().zip(word2.chars()) {
            if c1 != c2 && c2 != '.' {
                return false;
            }
        }
        true
    }
}