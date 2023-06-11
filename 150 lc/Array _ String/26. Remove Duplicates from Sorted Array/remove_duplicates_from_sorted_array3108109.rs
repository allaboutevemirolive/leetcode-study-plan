// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3108109/remove-duplicates-from-sorted-array-using-rust/
use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();

        for i in (0..n-1).rev() {
            if nums[i] == nums[i+1] {
                nums.remove(i+1);
            }
        }
        nums.len() as i32
    }
}