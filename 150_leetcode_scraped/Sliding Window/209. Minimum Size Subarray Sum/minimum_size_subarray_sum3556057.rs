// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/3556057/rust-beats-100/
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_length = (nums.len() + 1) as i32;
        let mut sum = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            sum += nums[right];

            while sum >= target {
                min_length = min_length.min((right - left + 1) as i32);
                sum -= nums[left];
                left += 1;
            }
        }

        if min_length == (nums.len() + 1) as i32 {
            return 0;
        }

        min_length
    }
}