// https://leetcode.com/problems/word-pattern/solutions/3033877/rust-with-hashmap-simple-solution/
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map_1 = HashMap::new();
        let mut map_2 = HashMap::new();
        let s = s.split_whitespace().into_iter().collect::<Vec<&str>>();
        if s.len() != pattern.len() {
            return false;
        }

        let pat = pattern.chars();

        for (idx, val) in pat.into_iter().enumerate() {
            if let Some(val2) = map_1.get(&val) {
                if !s[idx].eq(*val2) {
                    return false;
                }
            }

            if let Some(val1) = map_2.get(&s[idx]) {
                if *val1 != val {
                    return false;
                }
            }

            map_1.insert(val, s[idx]);
            map_2.insert(s[idx], val);
        }

        true
    }
}