// https://leetcode.com/problems/valid-palindrome/solutions/3191362/rust-0ms/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|x| x.is_alphanumeric())
            .map(|x| x.to_ascii_lowercase())
            .collect();
        s == s.chars().rev().collect::<String>()
    }
}