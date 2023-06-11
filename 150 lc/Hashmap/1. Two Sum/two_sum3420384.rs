// https://leetcode.com/problems/two-sum/solutions/3420384/rust-solution-in-o-n/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i,num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![i as i32, j];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }
}