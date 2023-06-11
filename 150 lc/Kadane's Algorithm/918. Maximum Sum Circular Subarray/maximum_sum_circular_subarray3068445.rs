// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3068445/kadane-s-algorithm-o-n-in-c-rust-go/
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut max, mut total, mut min) = (nums[0], 0, nums[0]);
        let (mut localMax, mut localMin) = (0, 0);
        for &num in nums.iter() {
            localMax = localMax.max(0) + num;
            max = max.max(localMax);
            total += num;
            localMin = localMin.min(0) + num;
            min = min.min(localMin);
        }
        if max < 0 { max } else { max.max(total - min) }
    }
}