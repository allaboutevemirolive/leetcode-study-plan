// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/3505339/rust-solution-0-ms-2-pointers/
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering::{Equal, Greater, Less};
        let (mut left, mut right) = (0, numbers.len() - 1);
        let mut last_possible_right = right;
        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Less => {left += 1; right = last_possible_right;}
                Greater => {right -= 1; last_possible_right = right;}
                Equal => break,
            }
        }
        vec![left as i32 + 1, right as i32 + 1]
    }
}