// https://leetcode.com/problems/valid-anagram/solutions/3272591/3ms-solution-in-rust/
use std::collections::HashMap;

fn count_chars(s: String) -> HashMap<char, usize> {
    let mut hashmap = HashMap::new();
    for i in s.chars() {
        match hashmap.get(&i) {
            Some(key) => { hashmap.insert(i, key + 1); }
            None => { hashmap.insert(i, 1); }
        }
    }
    hashmap
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        count_chars(s) == count_chars(t)
    }
}