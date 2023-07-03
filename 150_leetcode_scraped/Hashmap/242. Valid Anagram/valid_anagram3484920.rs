// https://leetcode.com/problems/valid-anagram/solutions/3484920/map-in-rust/
use std::iter::FromIterator;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut x = std::collections::HashMap::<char, i32>::new();

        for (a, b) in s.chars().zip(t.chars()) {
            *(x.entry(a).or_default()) += 1;
            *(x.entry(b).or_default()) -= 1;
        }
        x.into_values().all(|n| n == 0)
    }
}