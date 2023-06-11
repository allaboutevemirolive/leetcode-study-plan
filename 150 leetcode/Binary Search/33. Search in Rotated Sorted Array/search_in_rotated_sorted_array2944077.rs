// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2944077/most-common-rust-solution-rt-1ms-mem-2mb/
impl Solution {
	pub fn search(nums: Vec<i32,>, target: i32,) -> i32 {
		match nums.iter().enumerate().find(|&(i, &n,)| n == target,) {
			Some(i,) => i.0 as i32,
			None => -1,
		}
	}
}