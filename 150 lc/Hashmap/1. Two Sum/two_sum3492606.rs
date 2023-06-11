// https://leetcode.com/problems/two-sum/solutions/3492606/two-sum-memory-efficient-with-rust/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_len = nums.len();
        for (i, v1) in nums.iter().enumerate() {
            let start = i + 1;
            for (j, v2) in nums[start..nums_len].iter().enumerate() {
                if v1 + v2 == target {
                    return vec!(i as i32, (start + j) as i32);
                }
            }
        }
        vec!()
    }
}