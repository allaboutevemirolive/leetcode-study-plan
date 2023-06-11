// https://leetcode.com/problems/valid-palindrome/solutions/3470150/rust-o-n-solution/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let cleaned_input = s
            .as_bytes()
            .into_iter()
            .filter_map(|c| match c {
                c if c.is_ascii_alphanumeric() => Some(c.to_ascii_lowercase()),
                _ => None,
            })
            .collect::<Vec<_>>();
        if cleaned_input.len() == 0 {
            return true;
        }

        let (mut start, mut end) = (0, cleaned_input.len() - 1);
        while start < end {
            if cleaned_input[start] != cleaned_input[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}
