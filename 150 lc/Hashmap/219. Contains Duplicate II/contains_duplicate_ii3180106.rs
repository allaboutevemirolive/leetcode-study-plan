// https://leetcode.com/problems/contains-duplicate-ii/solutions/3180106/rust-using-hashmap/
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();

        for (i, num) in nums.into_iter().enumerate() {
            match map.get_mut(&num) {
                Some(idx) => {
                    if (i - *idx) as i32 <= k { return true; }
                    *idx = i;
                }
                None => { map.insert(num, i); }
            }
        }
        false
    }
}