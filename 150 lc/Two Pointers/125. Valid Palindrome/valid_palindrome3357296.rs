// https://leetcode.com/problems/valid-palindrome/solutions/3357296/recursion-solution-in-rust/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        help(s.as_bytes(), 0, s.len() - 1)
    }
}
pub fn help(s: &[u8], start: usize, end: usize) -> bool {
    if start >= end { return true; }
    let (left, right) = (s[start] as char, s[end] as char);
    let (is_start, is_end) = (left.is_ascii_alphanumeric(), right.is_ascii_alphanumeric());
    if !is_start || !is_end {
        return help(s, start + (!is_start as usize), end - (!is_end as usize));
    }
    if !right.to_lowercase().eq(left.to_lowercase()) {
        return false;
    }
    return help(s, start + 1, end - 1);
}