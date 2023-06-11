// https://leetcode.com/problems/valid-parentheses/solutions/3401235/100-faster-rust-code/
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let hashmap = HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}')
            ]);
        let mut stack = Vec::new();
        for i in s.chars(){
            match hashmap.get(&i){
                Some(value) => stack.push(value),
                _ => {
                    match stack.len(){
                        0 => return false,
                        n => {
                            if i != *stack[n - 1]{
                                return false
                            }
                            stack = stack[0..n-1].to_vec()
                        }
                    }
                }
            }
        }
        if stack.len() != 0{
            return false
        }
        true
    }
}