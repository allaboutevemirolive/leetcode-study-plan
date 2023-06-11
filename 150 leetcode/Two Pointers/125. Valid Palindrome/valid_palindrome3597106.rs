// https://leetcode.com/problems/valid-palindrome/solutions/3597106/rust-0ms-using-doubleendediterator/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s_chars = s.chars().filter(|&ch| ch.is_alphanumeric()).map(|ch| ch.to_ascii_lowercase());

        while let Some(front) = s_chars.next() {
            if let Some(back) = s_chars.next_back() {
                if front != back {
                    return false;
                }
            } else {
                return true;
            }
        }
        true
    }
}