// https://leetcode.com/problems/contains-duplicate-ii/solutions/2728369/rust-o-n-hashmap-soln/
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::<i32, usize>::new();
        let (k, len): (usize, usize) = (k as usize, nums.len());
        
        for i in 0..len {
            match map.get_mut(&nums[i]) {
                None => {
                    map.insert(nums[i], i);
                },
                Some(j) => {
                    if i - *j <= k {
                        return true
                    }
                    *j = i;
                }
            }            
        }
        
        false
    }
}