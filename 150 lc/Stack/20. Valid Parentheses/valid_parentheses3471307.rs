// https://leetcode.com/problems/valid-parentheses/solutions/3471307/simple-and-readable-rust-solution/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = vec![];
        for c in s.as_bytes() {
            match c {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b')' | b']' | b'}' if stack.pop() != Some(*c) => return false,
                _ => ()
            }
        }
        return stack.is_empty()
    }
}