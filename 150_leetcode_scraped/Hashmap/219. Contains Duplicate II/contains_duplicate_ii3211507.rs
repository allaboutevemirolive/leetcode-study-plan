// https://leetcode.com/problems/contains-duplicate-ii/solutions/3211507/rust-implementation/
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut lookup: HashMap<i32, usize> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            if let Some(value) = lookup.get(num) {
                if (index - value) <= (k as usize) {
                    return true;
                }
            }
            lookup.insert(*num, index);
        }

        false
    }

}