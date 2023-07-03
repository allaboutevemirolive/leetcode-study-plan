// https://leetcode.com/problems/palindrome-number/solutions/3414357/simple-rust-solution-beats-around-98-in-runtime-3ms/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let num = x.to_string();
        num == num.chars().rev().collect::<String>()
    }
}