// https://leetcode.com/problems/valid-anagram/solutions/3494374/rust-fold-solution/
use std::collections::HashMap;

fn count_chars(s: String) -> HashMap::<char, i32> {
    s.chars().fold(HashMap::<char, i32>::new(), |mut map, ch| {
        map.entry(ch).and_modify(|count| *count += 1 ).or_insert(1);
        map
    })
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        count_chars(s) == count_chars(t)
    }
}