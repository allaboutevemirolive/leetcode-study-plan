// https://leetcode.com/problems/valid-parentheses/solutions/3556940/super-simple-rust-solution-with-matching/
impl Solution {
    pub fn is_valid(s: String) -> bool {

        if s.len() <= 1 { return false };

        let mut brackets: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '{' | '(' | '[' => brackets.push(c),
                '}' | ')' | ']' => {
                    match brackets.pop() {
                        Some(top_c) => {
                            match (top_c, c) {
                                ('{', '}') | ('[', ']') | ('(', ')') => continue,
                                _ => return false,
                            }
                        },
                        None => return false,
                    }
                },
                _ => return false
            }
        }

        if brackets.len() >= 1 { return false}
        true
    }
}