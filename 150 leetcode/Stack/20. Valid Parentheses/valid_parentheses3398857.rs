// https://leetcode.com/problems/valid-parentheses/solutions/3398857/rust-simple-bracket-pattern-matching-with-vector-as-a-stack/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st: Vec<char> = Vec::new();
        for c in s.chars() {
            if ")}]".contains(c) {
                if st.len() == 0 {
                    return false;
                }
                let last_char = *st.last().unwrap();
                match (last_char, c) {
                    ('(', ')') | ('{', '}') | ('[', ']') => { st.pop(); },
                    (_, _) => { return false; },
                }
            } else {
                st.push(c);
            }
        }

        st.is_empty()
    }
}