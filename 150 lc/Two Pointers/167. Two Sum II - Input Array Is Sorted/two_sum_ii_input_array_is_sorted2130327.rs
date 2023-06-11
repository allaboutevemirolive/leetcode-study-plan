// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2130327/c-rust-o-n-time-0-1-extra-space/
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
		let mut left = 0;
		let mut right = numbers.len() - 1;
		loop {
			while numbers[left] + numbers[right] > target { 
				right -= 1;
			}
			if numbers[left] + numbers[right] == target {
				break vec![left as i32 + 1, right as i32 + 1];
			} else { 
				left += 1;
			}
		}
	}
}