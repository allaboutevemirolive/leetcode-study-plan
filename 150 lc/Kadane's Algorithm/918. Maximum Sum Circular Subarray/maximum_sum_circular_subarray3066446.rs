// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3066446/rust-dp-solution/
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut fmax = vec![0; n];
        let mut fmin = vec![0; n];

        let mut max = nums[0];
        let mut min = nums[0];
        let mut total = nums[0];

        fmax[0] = nums[0];
        fmin[0] = nums[0];

        for i in 1..n {
            fmax[i] = std::cmp::max(fmax[i - 1] + nums[i], nums[i]);
            fmin[i] = std::cmp::min(fmin[i - 1] + nums[i], nums[i]);

            max = std::cmp::max(max, fmax[i]);
            min = std::cmp::min(min, fmin[i]);

            total += nums[i];
        }


        if max > 0 { std::cmp::max(max, total - min) } else { max }
    }
}