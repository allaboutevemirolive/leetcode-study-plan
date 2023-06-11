// https://leetcode.com/problems/maximum-subarray/solutions/3161101/o-n-nice-rust-solution/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, i32::MIN) ,|(cur, mut max), &elem| {
            (0.max(elem + cur), max.max(elem + cur))
        }).1
    }
}