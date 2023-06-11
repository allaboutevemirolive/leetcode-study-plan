// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3068615/rust-calculate-max-subarray-and-min-subarray-by-using-kadane-s-algorithm/
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut acc_max = 0;
        let mut acc_min = 0;
        let mut max_array = i32::MIN;
        let mut min_array = i32::MAX;
        for n in nums {
            acc_max = n.max(n + acc_max);
            acc_min = n.min(n + acc_min);
            max_array = max_array.max(acc_max);
            min_array = min_array.min(acc_min);
            sum += n;
        }
        if sum != min_array {
            max_array.max(sum - min_array)
        } else {
            max_array
        }
    }
}