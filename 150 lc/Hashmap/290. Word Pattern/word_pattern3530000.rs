// https://leetcode.com/problems/word-pattern/solutions/3530000/bijective-mapping-in-word-pattern-matching-using-typescript-and-rust/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();

        if pattern.len() != words.len() { return false }

        let zipped_char_word = pattern.chars().zip(words.iter());

        let mut pattern_to_index = HashMap::new();
        let mut word_to_index = HashMap::new();
                  
        for (i, (character, &word)) in zipped_char_word.enumerate() {
            let pattern_index = pattern_to_index.entry(character).or_insert(i);
            let word_index = word_to_index.entry(word).or_insert(i);

            if pattern_index != word_index { return false; }
        }

        true
    }
}