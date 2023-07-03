// https://leetcode.com/problems/single-number/solutions/3370642/rust-one-line-xor/
impl Solution {
	pub fn single_number(nums: Vec<i32>) -> i32 {
		nums.iter().fold(0, |acc, v| acc ^ v)
	}
}