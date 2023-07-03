// https://leetcode.com/problems/is-subsequence/solutions/3316399/rust/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_iter = s.chars();
        let mut t_iter = t.chars();
        let mut s_char = s_iter.next();
        let mut t_char = t_iter.next();
        while s_char.is_some() && t_char.is_some() {
            if s_char == t_char {
                s_char = s_iter.next();
            }
            t_char = t_iter.next();
        }
        s_char.is_none()
    }
}