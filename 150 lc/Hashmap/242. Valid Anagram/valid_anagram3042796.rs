// https://leetcode.com/problems/valid-anagram/solutions/3042796/rust-compare-map-of-char-counts-linear-time/
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut char_counts = [0;26];
        s.chars().for_each(|c| char_counts[(c as u8 - b'a') as usize] += 1);
        for c in t.chars() {
            if char_counts[(c as u8 - b'a') as usize] == 0 {
                return false;
            }
            char_counts[(c as u8 - b'a') as usize] -= 1;
        }
        true
    }
}