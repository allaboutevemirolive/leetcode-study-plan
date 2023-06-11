// https://leetcode.com/problems/contains-duplicate-ii/solutions/2728175/rust-hashmap-one-liner-with-comments/
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        nums.into_iter().zip(0..).scan(HashMap::<i32, i32>::new(), |map, (num, i)| {
            match map.insert(num, i) {
                Some(j) => Some(i - j <= k),
                None => Some(false),
            }
        }).any(|ok| ok)
    }
}