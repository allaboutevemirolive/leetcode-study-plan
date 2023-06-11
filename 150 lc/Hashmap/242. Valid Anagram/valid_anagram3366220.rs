// https://leetcode.com/problems/valid-anagram/solutions/3366220/rust-hashmap/
impl Solution {
	pub fn is_anagram(s: String, t: String) -> bool {

		use std::collections::HashMap;
		let mut ds : HashMap<char, i32> = HashMap::new();
		let mut dt : HashMap<char, i32> = HashMap::new();
		for c in s.chars() {
			*ds.entry(c).or_insert(0) += 1;
		}
		for c in t.chars() {
			*dt.entry(c).or_insert(0) += 1;
		}

		return ds == dt;
	}
}