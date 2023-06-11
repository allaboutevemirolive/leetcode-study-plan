// https://leetcode.com/problems/valid-parentheses/solutions/3401161/clean-rust-o-n-solution/
impl Solution {
    fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' if stack.pop() != Some('(') => return false,
                ']' if stack.pop() != Some('[') => return false,
                '}' if stack.pop() != Some('{') => return false,
                _ => (),
            }
        }
        stack.is_empty()
    }
}