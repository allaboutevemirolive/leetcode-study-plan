// https://leetcode.com/problems/single-number/solutions/2838918/rust-the-old-xor-trick-using-fold/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |a, n| a ^ n)
    }
}