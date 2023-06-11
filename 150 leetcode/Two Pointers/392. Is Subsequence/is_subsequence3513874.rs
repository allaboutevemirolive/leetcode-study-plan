// https://leetcode.com/problems/is-subsequence/solutions/3513874/rust-iterator-solution-simple-to-understand/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        for c in s.chars() {
            // consume the `t` iterator to the first occurence of `c`
            match iter.find(|&p| p == c) {
                // continue the loop
                Some(_) => (),
                // `c` was not found in `t`
                None => return false
            }
        }
        true
    }
}