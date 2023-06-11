// https://leetcode.com/problems/summary-ranges/solutions/1805397/rust-solution/
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let a = nums[i];
            while i < nums.len() - 1 && nums[i + 1] == nums[i] + 1 {
                i += 1;
            }
            let b = nums[i];
            if a != b {
                result.push(format!("{}->{}", a, b));
            } else {
                result.push(format!("{}", a));
            };
            i += 1;
        }
        result
    }
}