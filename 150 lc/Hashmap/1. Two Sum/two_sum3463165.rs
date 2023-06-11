// https://leetcode.com/problems/two-sum/solutions/3463165/rust-linear-time-and-linear-space-solution-with-alternatives-and-complexity-analysis/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut value_to_index: HashMap<i32, i32> = Default::default();
        for i in 0..nums.len() {
            value_to_index.insert(nums[i], i as i32);
        }
        for i in 0..nums.len() {
            let offset = target - nums[i];
            if let Some(index) = value_to_index.get(&offset) {
                if *index != i as i32  {
                    return vec![i as i32, *index];
                }
            }
        }
        vec![]
    }
}