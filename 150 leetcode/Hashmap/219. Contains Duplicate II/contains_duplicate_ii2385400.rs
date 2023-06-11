// https://leetcode.com/problems/contains-duplicate-ii/solutions/2385400/rust-hashmap-minimal-code-o-n/
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut lookup:HashMap<i32,i32> = HashMap::new();
        for  (i,v) in nums.iter().enumerate() {
            let returned_val = lookup.insert(*v,i as i32);
            if returned_val != None {
                if ( i as i32 -  returned_val.unwrap()) <= k {
                    return true
                }            
            }
        }
        return false
    }   
}