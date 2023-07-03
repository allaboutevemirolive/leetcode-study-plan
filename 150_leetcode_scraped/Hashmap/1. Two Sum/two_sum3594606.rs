// https://leetcode.com/problems/two-sum/solutions/3594606/rust-hashmap-solution-beats-100/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pos_table: HashMap<i32, u32> = HashMap::new();
        let mut soln: Vec<i32> = vec![0, 0];
        // Create iterator over input vector
        // Enumerate that iterator to get (idx, number) pairs
        // use for_each() to check each (idx, number) pair for it's compliment in the HashMap
        nums.iter().enumerate().for_each(|(idx, number)| {
            // Checks to see if the compliment is in the HashMap
            if let Some(position) = pos_table.get(&(target - number)) {
                soln = vec![idx as i32, *position as i32];
            }
            // Insert the current number and its index into the HashMap
            // This can be used for future lookups if the current one is unsuccessful
            pos_table.insert(*number, idx as u32);
        });
        return soln;
    }
}