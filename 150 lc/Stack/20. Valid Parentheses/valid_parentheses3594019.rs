// https://leetcode.com/problems/valid-parentheses/solutions/3594019/simple-rust-matching/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }

        let mut v = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => v.push(c),
                _ => match v.pop() {
                    Some('(') if c == ')' => (),
                    Some('[') if c == ']' => (),
                    Some('{') if c == '}' => (),
                    _ => return false,
                    }
            }
        }

        v.is_empty()
    }
}