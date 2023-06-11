// https://leetcode.com/problems/maximum-subarray/solutions/3315652/rust-dynamic-programming-kadane-s-algorithm-concise-solution/
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter().fold((i32::MIN, 0), |(maxsum, cursum), &num| {
            let cursum = max(cursum + num, num);
            (max(maxsum, cursum), cursum)
        }).0
    }
}