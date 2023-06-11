// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3186881/3-line-rust-solution/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(_) => vec![nums.partition_point(|&x| x < target) as i32, nums.partition_point(|&x| x <= target) as i32 - 1],
            Err(_) => vec![-1, -1]
        }
    }
}