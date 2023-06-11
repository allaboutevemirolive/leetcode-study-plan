// https://leetcode.com/problems/valid-palindrome/solutions/3160944/clean-rust-solution/
pub fn is_palindrome(s: String) -> bool {
    let mut s = s.to_ascii_lowercase();
    s.retain(|c| c.is_ascii_alphanumeric());
    s.chars().rev().collect::<String>() == s
}