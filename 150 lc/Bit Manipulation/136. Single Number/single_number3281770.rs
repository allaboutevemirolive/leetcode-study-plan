// https://leetcode.com/problems/single-number/solutions/3281770/rust-1-line/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, v| acc ^ v)
    }
}