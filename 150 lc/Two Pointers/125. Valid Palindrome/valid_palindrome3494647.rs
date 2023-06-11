// https://leetcode.com/problems/valid-palindrome/solutions/3494647/rust-filter-map-zip/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let head = s.chars().filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        });

        let tail = head.clone().rev();

        head.zip(tail).all(|(h, t)| h == t)
    }
}