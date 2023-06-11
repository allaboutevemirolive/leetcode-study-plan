// https://leetcode.com/problems/maximum-subarray/solutions/3179742/simple-rust-solution/
use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut maxsum = sum;
        if nums.len() == 1 {
            return nums[0];
        }
        for i in 1..nums.len() {
            sum += nums[i];
            sum = cmp::max(sum, nums[i]);
            // if sum < nums[i] {
            //     sum = nums[i]
            // }
            maxsum = cmp::max(maxsum, sum);
            // if maxsum < sum {
            //     maxsum = sum
            // }            
        }
        return maxsum
    }
}