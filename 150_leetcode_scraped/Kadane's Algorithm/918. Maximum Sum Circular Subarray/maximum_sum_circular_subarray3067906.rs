// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3067906/rust-variation-of-kadane-s-algo/
 impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut running_sum = 0;
        let mut local_max   = 0;
        let mut local_min   = 0;
        let mut global_max  = i32::MIN;
        let mut global_min  = i32::MAX;

        for num in nums {
            running_sum += num;
            local_max   = num.max(num + local_max);
            local_min   = num.min(num + local_min);
            global_max  = global_max.max(local_max);
            global_min  = global_min.min(local_min);
        }
        if running_sum == global_min {
            global_max
        } else {
            global_max.max(running_sum - global_min)
        }
    }
}