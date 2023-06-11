// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2901889/rust-0ms-2mb-with-loops-duh/
use std::{io::Read, mem};

const CHARS: &[&[char]] = &[
    &['a', 'b', 'c'],
    &['d', 'e', 'f'],
    &['g', 'h', 'i'],
    &['j', 'k', 'l'],
    &['m', 'n', 'o'],
    &['p', 'q', 'r', 's'],
    &['t', 'u', 'v'],
    &['w', 'x', 'y', 'z'],
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut out = Vec::new();

        for digit in digits.chars() {
            let digit = digit.to_digit(10).unwrap() as usize;

            if out.is_empty() {
                out = CHARS[digit - 2].into_iter().map(|c| c.to_string()).collect();
                continue;
            }

            mem::take(&mut out).into_iter().for_each(|s| CHARS[digit - 2].into_iter().for_each(|c| out.push(format!("{s}{c}"))));
        }

        out
    }
}