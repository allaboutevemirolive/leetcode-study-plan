// https://leetcode.com/problems/maximum-subarray/solutions/2581487/rust-solution-o-n/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0].clone();
        let mut current_sum = 0;
        for value in &nums {
            current_sum = if current_sum < 0 {
                value.clone()
            }
            else {
                current_sum + value
            };
            if current_sum >= max_sum {
                max_sum = current_sum;
            }
        }
        max_sum
    }
}