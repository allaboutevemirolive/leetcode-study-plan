// https://leetcode.com/problems/valid-palindrome/solutions/3216555/rust-with-iterators-and-filter-on-is-alphanumeric/

impl Solution {
    pub fn is_palindrome(s: String) -> bool {

        let s_clean: String = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();

        for (left,right) in s_clean.chars().rev().zip(s_clean.chars()){
            if left != right {
                return false;
            }
        }

        true
        
    }
}