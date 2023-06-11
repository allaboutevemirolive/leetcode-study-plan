// https://leetcode.com/problems/word-pattern/solutions/2984219/rust-1-liner-with-some-linting/
use std::collections::HashSet;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        s.split(" ")
            .zip(pattern.as_bytes())
            .collect::<HashSet<(&str, &u8)>>()
            .len()
            == s.split(" ").collect::<HashSet<&str>>().len()
            && s.split(" ").collect::<Vec<&str>>().len() == pattern.as_bytes().len()
            && s.split(" ").collect::<HashSet<&str>>().len()
                == pattern.as_bytes().iter().collect::<HashSet<&u8>>().len()
    }
}
