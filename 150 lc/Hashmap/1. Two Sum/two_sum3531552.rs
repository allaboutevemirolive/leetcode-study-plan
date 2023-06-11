// https://leetcode.com/problems/two-sum/solutions/3531552/rust-twosum-moment/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 1..nums.len() {
            for j in i..nums.len() {
                if nums[j] + nums[j-i] == target {
                    return vec![(j-i) as i32, j as i32]
                }
            }
        }
        return vec![-1, -1];
    }
}