// https://leetcode.com/problems/two-sum/solutions/3562341/rust-beats-100/
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..nums.len() {
            let entry = map.entry(nums[i]).or_insert(vec![]);
            entry.push(i as i32);
        }

        for i in 0..nums.len() {
            let diff = target - nums[i];

            if let Some(value) = map.get(&diff) {
                if value.len() > 1 {
                    return vec![i as i32, value[1]];
                } else if diff != nums[i] {
                    return vec![i as i32, value[0]];
                }
            }
        }

        vec![-1, -1]
    }
}