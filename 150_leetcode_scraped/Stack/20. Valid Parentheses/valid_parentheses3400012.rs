// https://leetcode.com/problems/valid-parentheses/solutions/3400012/rust-100-faster-using-match-statements/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            // if its opening
            match ch {
                '(' => stack.push(ch),
                '[' => stack.push(ch),
                '{' => stack.push(ch), 
                _ => {
                    match stack.pop() {
                        // closing parantheses
                        None => { return false; },
                        Some(val) => {
                            match val {
                                '(' => { if ch != ')' { return false; } },
                                '[' => { if ch != ']' { return false; } },
                                '{' => { if ch != '}' { return false; } }, 
                                _ => {} 
                            } 
                        } 
                    }
                },
            }
        }

        return stack.len() == 0;
    }
}