// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/2278595/rust-sliding-window-o-n-binary-search-o-nlogn/
impl Solution {
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
	let mut sum = 0;
	let mut i = 0;
	let mut min  =  100000000;
	for j in 0..nums.len() {
		sum += nums[j];
		if sum >= target  && min > j-i {
			min = j-i;
		}
		while i<=j && sum>=target{
			sum -= nums[i];
			i += 1;
			if sum >= target  && min > j-i{
				min = j-i;
			}
		}
	}
	match min {
		100000000 => 0,
		_ => (min + 1) as i32
	}
}
}