// https://leetcode.com/problems/word-pattern/solutions/2981461/rust-hashmap-zip-clean-code-explained/
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut c_to_w: HashMap<char, &str> = HashMap::new();
        let mut w_to_c: HashMap<&str, char> = HashMap::new();

        let chars = pattern.chars();
        let words = s.split_whitespace();

        pattern.len() == words.clone().count()
        && chars.zip(words).all(|(c, w)| {
            *c_to_w.entry(c).or_insert(w) == w
            && *w_to_c.entry(w).or_insert(c) == c
        })
    }
}