// https://leetcode.com/problems/single-number/solutions/3513351/rust-functional-approach-with-description/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| acc^x)
    }
}