// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2130016/rust-two-pointers-three-variations/
use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_it = numbers.iter().enumerate().peekable();
        let mut right_it = numbers.iter().enumerate().rev().peekable();

        while let (Some((left_index, left_value)), Some((right_index, right_value))) = (left_it.peek(), right_it.peek()) {
            match (*left_value + *right_value).cmp(&target) {
                Ordering::Less => { left_it.next(); },
                Ordering::Equal => return vec![*left_index as i32 + 1, *right_index as i32 + 1],
                Ordering::Greater => { right_it.next(); },
            }
        }
        vec![]
    }
}