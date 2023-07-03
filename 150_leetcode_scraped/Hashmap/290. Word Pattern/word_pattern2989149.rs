// https://leetcode.com/problems/word-pattern/solutions/2989149/rust-0ms-100-faster-simple-and-easy/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map = HashMap::with_capacity(3000);
        let mut set = HashSet::with_capacity(3000);
        let v = s.trim().split_ascii_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        if v.len() != pattern.len() {
            return false;
        }
        for (i, c) in pattern.chars().enumerate() {
            if map.contains_key(&c) && *map.get(&c).unwrap() != &v[i] {
                return false;
            }
            if !map.contains_key(&c) && set.contains(&v[i]) {
                return false;
            }
            if !map.contains_key(&c) && !set.contains(&v[i]) {
                map.insert(c, &v[i]);
                set.insert(&v[i]);
            }
        }
        true
    }
}