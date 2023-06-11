// https://leetcode.com/problems/single-number/solutions/2992842/rust-iterator/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |result, num| num ^ result)
    }
}