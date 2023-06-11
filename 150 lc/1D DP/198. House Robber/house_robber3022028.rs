// https://leetcode.com/problems/house-robber/solutions/3022028/rust-dynamic-programming-bottom-up-iteration-with-constant-space/
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, 0), | (n_minus_2, n_minus_1), x| (n_minus_1, max(x + n_minus_2, n_minus_1))).1
    }
}