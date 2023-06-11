// https://leetcode.com/problems/maximum-subarray/solutions/2840989/rust-maximum-subarray/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        let mut result = dp[0];

        for i in 1..nums.len() {
            if dp[i - 1] > 0 {
                dp[i] = dp[i - 1] + nums[i];
            } else {
                dp[i] = nums[i];
            }
            result = result.max(dp[i]);
        }
        result
    }
}