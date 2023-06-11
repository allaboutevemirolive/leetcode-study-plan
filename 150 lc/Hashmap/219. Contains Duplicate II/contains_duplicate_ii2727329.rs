// https://leetcode.com/problems/contains-duplicate-ii/solutions/2727329/rust-with-hashset/
use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hash = HashSet::new();
        for i in 0..nums.len() {
            if i > k as usize {
                hash.remove(&nums[i - k as usize - 1]);
            }
            if !hash.insert(nums[i]) {
                return true;
            }
        }
        false
    }
}