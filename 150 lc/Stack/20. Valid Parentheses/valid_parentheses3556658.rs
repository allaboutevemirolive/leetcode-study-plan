// https://leetcode.com/problems/valid-parentheses/solutions/3556658/rust-super-straightforward-solution/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn is_opening_char(c: char) -> bool {
            return c == '(' || c == '{' || c == '['
        }
        
        fn is_closing_char(c: char) -> bool {
            return c == ')' || c == '}' || c == ']'
        }

        fn is_proper_match(open: char, close: char) -> bool {
            return (open == '(' && close == ')') || (open == '{' && close == '}') || (open == '[' && close == ']')
        }

        let mut stack: Vec<char> = vec![];
        
        for c in s.chars() {
            if is_opening_char(c) {
                stack.push(c)
            } else if is_closing_char(c) {
                if stack.is_empty() {
                    return false
                } 
                
                if !is_proper_match(stack[stack.len() - 1], c) {
                    return false
                } 
                stack.pop();
            } else {
                panic!("Unexpected state")
            }
        }

        return stack.is_empty()
    }
}