// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/3542163/1ms-rust-solution-using-vecdeque/
use std::collections::VecDeque;

impl Solution {
    pub fn values(c: char) -> Vec<char> {
        match c {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => panic!("invalid input"),
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut partials = VecDeque::new();
        for i in 0..digits.chars().count() {
            let digit = digits.chars().nth(i).unwrap();
            if partials.len() == 0 {
                Solution::values(digit)
                    .into_iter()
                    .for_each(|v| partials.push_back(v.to_string()));
                continue;
            }

            for _ in 0..partials.len() {
                let partial = partials.pop_front().unwrap();
                for value in Solution::values(digit) {
                    partials.push_back(format!("{partial}{value}"))
                }
            }
        }
        partials.into_iter().collect()
    }
}