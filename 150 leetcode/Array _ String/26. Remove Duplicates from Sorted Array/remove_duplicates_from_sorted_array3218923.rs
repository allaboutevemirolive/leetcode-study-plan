// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3218923/rust-this-is-hilariously-easy/
use std::convert::TryInto;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            nums.dedup();
            nums.len().try_into().unwrap()
    }
}