// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/2580123/rust-one-pass/
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut min_curr, mut max_curr, mut min, mut max, mut sum) = (0, 0, i32::MAX, i32::MIN, 0);
        for &num in nums.iter() {
            min_curr = num.min(min_curr + num);
            min = min.min(min_curr);
            max_curr = num.max(max_curr + num);
            max = max.max(max_curr);
            sum += num;
        }
        if max > 0 { max.max(sum - min) } else { max }
    }
