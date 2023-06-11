// https://leetcode.com/problems/happy-number/solutions/3068754/rust-short-and-simple-0-ms-2-mb/
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut duplicate: HashSet<i32> = HashSet::new();

        loop {
            if n == 1 { return true; }
            if !duplicate.insert(n) { return false; }
            n = n.to_string().chars().fold(0, |acc, c| acc + (c as i32 - 48).pow(2));
        }
    }
}