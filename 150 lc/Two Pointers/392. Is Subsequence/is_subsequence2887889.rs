// https://leetcode.com/problems/is-subsequence/solutions/2887889/subsequence-using-stack-logic-in-rust/
impl Solution {
    pub fn is_subsequence(mut s: String, mut t: String) -> bool {
        while !t.is_empty() && !s.is_empty() {
            if t.pop().unwrap() == s.chars().last().unwrap() {
                s.pop();
            }
        }
        s.is_empty()
    }
}