// https://leetcode.com/problems/two-sum/solutions/3455006/rust/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut map = HashMap::new();

       for (i, &num) in nums.iter().enumerate() {
           let complement = target - num;

           if let Some(&j) = map.get(&complement) {
               return vec![i as i32, j as i32];
           }
           map.insert(num, i);
       }
       vec![]
    }
}