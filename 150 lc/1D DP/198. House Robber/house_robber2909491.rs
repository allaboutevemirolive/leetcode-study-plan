// https://leetcode.com/problems/house-robber/solutions/2909491/1-line-of-functional-rust-o-n-time-o-1-memory/
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, 0), |(p1, p2), c| (max(c + p2, p1), p1)).0
    }
}