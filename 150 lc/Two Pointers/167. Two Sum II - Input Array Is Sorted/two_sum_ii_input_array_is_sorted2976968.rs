// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2976968/rust-beats-100-two-pointer/
use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) : (i32, i32) = (0, numbers.len() as i32 - 1);
        while l < r {
            let cursum = numbers[l as usize] + numbers[r as usize];
            match cursum.cmp(&target) {
                Ordering::Equal => return vec![l+1, r+1],
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            };
        }
        unreachable!()
    }
}