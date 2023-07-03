// https://leetcode.com/problems/search-insert-position/solutions/3482994/rust-beginner-binary-search-solution/
use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target < nums[0] {
            return 0;
        }

        if target > nums[nums.len() - 1] {
            return nums.len() as i32;
        }

        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mut pivot = left + (right - left) / 2;
            match target.cmp(&nums[pivot]) {
                Ordering::Equal => return pivot as i32,
                Ordering::Less => right = pivot - 1,
                Ordering::Greater => left = pivot + 1
            }
        }

        left as _
    }
}