// https://leetcode.com/problems/valid-parentheses/solutions/3414388/rust-solution/
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let matchmap: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')].iter().cloned().collect();
        let openers: HashSet<char> = ['(', '[', '{'].iter().cloned().collect();
        let mut stack: Vec<char> = Vec::new();
        
        for c in s.chars() {
            if openers.contains(&c) {
                stack.push(c);
            } else if let Some(&last) = stack.last() {
                if matchmap.get(&c) == Some(&last) {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        stack.is_empty()
    }
}