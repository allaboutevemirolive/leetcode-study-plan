// https://leetcode.com/problems/valid-parentheses/solutions/3398852/rust-using-hashmap/
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let m: HashMap<char, char> = [(')', '('), ('}', '{'), (']', '['), ('(', ' '), ('{', ' '), ('[', ' ')].iter().cloned().collect();

        for l in s.chars() {
            if let Some(&v) = m.get(&l) {
                if stack.len() > 0 && stack[stack.len() - 1] == v {
                    stack.pop();
                } else {
                    stack.push(l);
                }
            }
        }

        if stack.len() == 0 { true } else { false }
    }
}