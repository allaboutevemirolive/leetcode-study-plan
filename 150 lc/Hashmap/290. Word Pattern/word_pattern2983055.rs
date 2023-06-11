// https://leetcode.com/problems/word-pattern/solutions/2983055/o-n-simple-rust/
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut dp: HashMap<char, String> = HashMap::new();
        let list: Vec<&str> = s.split(" ").collect();
        if list.len() != pattern.len() {
            return false;
        }
        for (i, letter) in pattern.chars().into_iter().enumerate() {
            if let Some(value) = dp.get_key_value(&letter) {
                if value != (&letter, &list[i].to_owned()) {
                    return false;
                }
            } else {
                if let Some(_) = dp.values().find(|x| x == &list[i]) {
                    return false;
                }
                dp.insert(letter, list[i].to_string());
            }
        }
        true
    }
}