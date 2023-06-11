// https://leetcode.com/problems/palindrome-number/solutions/3374866/rust-very-basic-solution/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().collect::<String>() == x.to_string().chars().rev().collect::<String>()
    }
}