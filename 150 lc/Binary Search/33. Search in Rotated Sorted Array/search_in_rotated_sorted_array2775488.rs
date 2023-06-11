// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2775488/rust-nested-match/
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        #[inline(always)]
        fn is_rotated(num_slice: &[i32]) -> bool {
            num_slice.first() > num_slice.last()
        }

        #[inline(always)]
        fn is_in_bounds(sorted_slice: &[i32], target: i32) -> bool {
            sorted_slice.first() <= Some(&target) && Some(&target) <= sorted_slice.last()
        }

        if !is_rotated(&nums[..]) && !is_in_bounds(&nums[..], target) {
            return -1;
        }

        let (mut l, mut r) = (0, nums.len()-1);

        while l <= r {
            let m = l + (r - l) / 2;
            match nums[m].cmp(&target) {
                Ordering::Equal => return m as i32,
                _ => {
                    match is_rotated(&nums[l..=m]) {
                        false => match is_in_bounds(&nums[l..=m], target) {
                            true => r = m - 1,
                            false => l = m + 1,
                        },
                        true => match is_in_bounds(&nums[m+1..=r], target) {
                            true => l = m + 1,
                            false => r = m - 1,
                        }
                    }                    
                }
            };
        }
        -1
    }
}