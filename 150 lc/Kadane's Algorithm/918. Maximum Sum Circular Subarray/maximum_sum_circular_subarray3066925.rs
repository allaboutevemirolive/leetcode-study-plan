// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3066925/rust-lee215-s-one-pass-solution/
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let (mut maxSum, mut curMax) = (nums[0], 0);
        let (mut minSum, mut curMin) = (nums[0], 0);
        for a in nums {
            curMax = a.max(curMax + a);
            maxSum = maxSum.max(curMax);
            curMin = a.min(curMin + a);
            minSum = minSum.min(curMin);
            total += a;
        }
        if maxSum > 0 {
            return maxSum.max(total - minSum);
        }
        maxSum
    }
}