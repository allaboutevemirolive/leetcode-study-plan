// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/3561649/rust-reusing-solution-to-162-find-peak-element/
use std::cmp::Ordering::{Greater, Less, Equal};

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let k = Self::find_peak(&nums);
        nums[k%n]
    }

    fn find_peak(nums: &Vec<i32>) -> usize {
        let (mut lbound, mut ubound) = (0, nums.len());
        while lbound < ubound {
            let mid = (lbound+ubound)/2;
            match nums[mid].cmp(&nums[0]) {
                Greater|Equal => {lbound = mid+1;}
                Less => {ubound = mid;}
            }
        }
        ubound
    }
}