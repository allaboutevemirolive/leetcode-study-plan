// https://leetcode.com/problems/house-robber/solutions/2824465/2-rust-clean-and-simple-solutions-linear-time-constant-space/
pub fn rob(nums: Vec<i32>) -> i32 {
	use std::cmp::max;
	let (mut a, mut b) = (0, 0);
	for num in nums {
		let best = max(a + num, b);
		a = b;
		b = best;
	}
	max(a, b)
}