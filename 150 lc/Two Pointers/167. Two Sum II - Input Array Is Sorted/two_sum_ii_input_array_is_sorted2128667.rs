// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2128667/rust-binary-search-then-two-pointers/
use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hi = match numbers.binary_search(&(target - numbers[0])) {
            Ok(x) => return vec![1, x as i32 + 1],
            Err(x) => x - 1,
        };
        let mut lo = if hi == numbers.len() - 1 {
            match numbers[..hi].binary_search(&(target - numbers[hi])) {
                Ok(x) => return vec![x as i32 + 1, hi as i32 + 1],
                Err(x) => x,
            }
        }
        else {
            1
        };
        loop {
            match (numbers[lo] + numbers[hi]).cmp(&target) {
                Ordering::Equal => return vec![lo as i32 + 1, hi as i32 + 1],
                Ordering::Less => lo += 1,
                Ordering::Greater => hi -= 1,
            }
        }
    }
}