// https://leetcode.com/problems/valid-palindrome/solutions/3516914/rust-solution-using-filter-map/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let str = s.chars()
            .filter_map(|c| {
                if c.is_alphanumeric() { Some(c.to_ascii_lowercase()) }
                else { None }
            });
        
        str.clone().eq(str.rev())
    }
}
