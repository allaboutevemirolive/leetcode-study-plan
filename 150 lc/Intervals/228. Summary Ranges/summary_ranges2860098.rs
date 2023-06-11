// https://leetcode.com/problems/summary-ranges/solutions/2860098/rust-solution/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
	if nums.is_empty() {
		return vec![];
	}

	let mut idx = 1;
	let mut check = (nums[0], nums[0]);
	let mut tmps = vec![];
	let mut res = vec![];

	while idx < nums.len() {
		if check.1 + 1 != nums[idx] {
			check.1 = nums[idx - 1];
			tmps.push(check);
			check = (nums[idx], nums[idx]);
		} else {
			check.1 = nums[idx];
		}

		idx += 1;
	}

	tmps.push(check);

	for tmp in tmps {
		if tmp.0 == tmp.1 {
			res.push(tmp.0.to_string());
		} else {
			res.push(tmp.0.to_string() + "->" + &tmp.1.to_string());
		}
	}

	res
}