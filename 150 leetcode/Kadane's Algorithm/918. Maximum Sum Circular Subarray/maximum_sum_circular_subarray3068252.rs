// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3068252/rust-solution/
use std::cmp;

impl Solution {

    fn kadane(nums: &Vec<i32>) -> i32{
        let mut local = nums[0];
        let mut global = nums[0];

        for i in 1..nums.len() {
            local = cmp::max(nums[i], nums[i] + local);
            global = cmp::max(local, global);            
        }

        global
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let max_subarray = Self::kadane(&nums);

        if max_subarray < 0 {
            return max_subarray;
        }

        let total_sum : i32 = nums.iter().sum();

        let mut nums = nums.clone();

        for i in 0..n {
            nums[i] = -nums[i];
        }

        let min_subarray = Self::kadane(&nums);

        cmp::max(max_subarray, total_sum + min_subarray)
    }
}