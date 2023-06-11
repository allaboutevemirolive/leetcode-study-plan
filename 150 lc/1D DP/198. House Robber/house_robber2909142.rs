// https://leetcode.com/problems/house-robber/solutions/2909142/rust-dp-in-place-top-down-o-n/
use std::cmp::max;
impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if (nums.len() == 1) {
            return nums[0];
        }
        nums[1] = max(nums[0], nums[1]);
        for i in 2..=(nums.len()-1) {
            nums[i] = max(nums[i]+nums[i-2], nums[i-1]);
        }
        return nums[nums.len()-1];
    }
}