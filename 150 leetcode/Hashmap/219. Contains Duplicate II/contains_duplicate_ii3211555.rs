// https://leetcode.com/problems/contains-duplicate-ii/solutions/3211555/rust-implementation-using-hashset/
use std::collections::HashSet;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut lookup: HashSet<i32> = HashSet::new();

        let k = k as usize;
        
        for i in 0..nums.len() {
            if i > k {
                lookup.remove(&nums[i-k-1]);
            }
            if !lookup.insert(nums[i]) { 
                return true;
            }
        }

        false
    }
}