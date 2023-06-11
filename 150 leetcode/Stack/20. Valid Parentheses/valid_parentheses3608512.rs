// https://leetcode.com/problems/valid-parentheses/solutions/3608512/100-faster-rust-solution/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if (stack.len() != 0 && (stack[stack.len() - 1] == c)) {
                        stack.pop();
                    } else {
                        return false;
                    }
                },
                _ => unreachable!()
            };
        }

        stack.len() == 0
    }
}