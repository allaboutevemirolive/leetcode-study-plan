// https://leetcode.com/problems/two-sum/solutions/3492762/rust-hashmap-solution/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            map.insert(*v, i);
        }
        for (i, v) in nums.iter().enumerate() {
            let res = map.get(&(target - v));
            if res.is_some() {
                let j_value = *res.unwrap();
                if i != j_value {
                    return vec!(i as i32, j_value as i32)
                }
            }
        }
        vec!()
    }
}