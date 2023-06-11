// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/3360492/rust-simple-solution-using-fifo-0ms-runtime/
use std::{collections::VecDeque, fmt::format};

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }
    
    let mut dequeue: VecDeque<String> = VecDeque::new();
    let pad: [&str; 10] = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    dequeue.push_back(String::from(""));

    for (index, c) in digits.char_indices() {
        let digit = c.to_digit(10).unwrap() as usize;
        
        let digit_for_pad = pad[digit];
        
        while dequeue.front().unwrap().len() == index {
            let element = dequeue.pop_front().unwrap();
            for x in digit_for_pad.chars() {
                dequeue.push_back( format!("{}{}", element, x));
            }
        }
    }

    return dequeue.into_iter().collect();
}
}