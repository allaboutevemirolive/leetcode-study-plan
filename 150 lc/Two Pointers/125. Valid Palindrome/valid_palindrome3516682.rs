// https://leetcode.com/problems/valid-palindrome/solutions/3516682/rust-0ms-functional-approach-two-line/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.chars().filter(|ch| ch.is_alphanumeric()).collect::<String>();
        s.chars().zip(s.chars().rev()).take(s.len()/2+1).all(|(l,r)| l.eq_ignore_ascii_case(&r))
    }
}