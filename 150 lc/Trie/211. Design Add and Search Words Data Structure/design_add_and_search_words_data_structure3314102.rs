// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3314102/rust-simple-brute-force/
use std::collections::HashSet;
struct WordDictionary {
    words: Vec<Vec<String>>,
}

impl WordDictionary {
    fn new() -> Self {
        Self { words: vec![Vec::new(); 26] }
    }
    
    fn add_word(&mut self, word: String) {
        self.words[word.len()].push(word);
    }
    
    fn search(&self, word: String) -> bool {
        self.words[word.len()]
            .iter()
            .any(|w| {
                w.as_bytes().iter()
                    .zip(word.as_bytes().iter())
                    .all(|(&a,&b)| b == b'.' || a == b)
            })
    }
}