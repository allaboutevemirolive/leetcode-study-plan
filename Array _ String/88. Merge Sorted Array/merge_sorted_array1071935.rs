// https://leetcode.com/problems/merge-sorted-array/solutions/1071935/rust-solution/
use std::convert::TryInto;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {        
        nums1.split_off(nums1.len() - n as usize);
        nums1.append(nums2);
        nums1.sort();
    }
}