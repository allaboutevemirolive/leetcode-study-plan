// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/1479788/rust-2-lines-sort-soltuion-binary-heap/
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {

         nums.sort();
         nums[nums.len() as usize - k as usize]
	}
}