// https://leetcode.com/problems/isomorphic-strings/solutions/3408710/0ms-straightforward-rust-using-hashmap-char-char-and-match/
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_dict: HashMap<char, char> = HashMap::new();
        let mut t_dict: HashMap<char, char> = HashMap::new();
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();
        for index in 0..s_vec.len() {
            let s_char: char = s_vec[index];
            let t_char: char = t_vec[index];
            match s_dict.get(&s_char) {
                Some(s_item) => {
                    if s_item != &t_char { return false };
                },
                None => {
                    s_dict.insert(s_char, t_char);
                }
            };
            match t_dict.get(&t_char) {
                Some(t_item) => {
                    if t_item != &s_char { return false };
                },
                None => {
                    t_dict.insert(t_char, s_char);
                }
            };
        } 
        true
    }
}