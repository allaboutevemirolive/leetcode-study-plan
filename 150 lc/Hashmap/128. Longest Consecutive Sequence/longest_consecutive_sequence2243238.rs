// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2243238/one-pass-solution-joining-intervals-hashmap-benchmarks/
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
	use std::collections::HashMap;

	let n = nums.len();
	nums         
		.into_iter()
		.fold((HashMap::with_capacity(n), 0), |(mut sequences, ans), num| {
			let (mut prev_start, mut next_end) = (*sequences.get(&(num - 1)).unwrap_or(&0), *sequences.get(&(num + 1)).unwrap_or(&0));
			let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
			if !sequences.contains_key(&num) {
				match (has_prev, has_next) {
					(true, true) => {
						*sequences.get_mut(&prev_start).unwrap() = next_end;
						*sequences.get_mut(&next_end).unwrap() = prev_start;
						sequences.insert(num, num);
					},
					(true, false) => {
						sequences.insert(num, prev_start);
						*sequences.get_mut(&prev_start).unwrap() = num;
						next_end = num;
					},
					(false, true) => {
						sequences.insert(num, next_end);
						*sequences.get_mut(&next_end).unwrap() = num;
						prev_start = num;
					},
					(false, false) => {sequences.insert(num, num);}
				};
				(sequences, ans.max(i32::abs(next_end - prev_start) + 1))
			} else {
				(sequences, ans.max(1))
			}
		})
		.1
}