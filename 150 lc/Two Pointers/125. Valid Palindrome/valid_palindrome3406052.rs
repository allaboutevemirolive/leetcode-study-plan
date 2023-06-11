// https://leetcode.com/problems/valid-palindrome/solutions/3406052/rust-two-pointers-100/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = chars.len() - 1;
        while start < end {
            if !chars[start].is_ascii_alphanumeric() {
                start += 1;
                continue;
            }
            if !chars[end].is_ascii_alphanumeric() {
                end -= 1;
                continue;
            }
            if chars[start].to_ascii_lowercase() != chars[end].to_ascii_lowercase() {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}