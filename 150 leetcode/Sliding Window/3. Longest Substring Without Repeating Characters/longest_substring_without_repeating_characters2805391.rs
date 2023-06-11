// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/2805391/rust-two-pointers-o-n/
use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ci = HashMap::new();
        let mut ans = 0;
        let mut l = 0;
        for (i, c) in s.chars().enumerate() {
            if ci.contains_key(&c) {
                l = max(ci[&c] + 1, l);
                let curt = i - l + 1;
                ans = max(ans, curt);
            } else {
                let curt = i - l + 1;
                ans = max(ans, curt)
            }
            *ci.entry(c).or_insert(i) = i;
            // println!("i {i} c {c}");
        }
        ans as i32
    }
}