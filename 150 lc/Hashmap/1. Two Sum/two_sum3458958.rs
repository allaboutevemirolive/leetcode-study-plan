// https://leetcode.com/problems/two-sum/solutions/3458958/rust-beats-100-with-hashmap-and-functional-style/
use std::collections::HashMap;

impl Solution {
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

let mut seenNums = HashMap::new();

    nums.iter().enumerate().find_map(|(i, n)| {
        let isExist = target - n;

        match seenNums.get(&isExist)  {
            Some(n) => Some(vec![*n, i as i32]),
            _ => {
                seenNums.insert(n,  i as i32);
                None
            }
        }
    }).unwrap()
}
}