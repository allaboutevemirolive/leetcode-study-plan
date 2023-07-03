// https://leetcode.com/problems/house-robber/solutions/2911384/rust-dp-with-o-1-space-one-line-function/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold((0, 0), |acc, item|  (acc.1, std::cmp::max(acc.0+item, acc.1)) ).1
    }
}