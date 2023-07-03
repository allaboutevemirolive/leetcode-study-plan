// https://leetcode.com/problems/merge-sorted-array/solutions/2122619/rust-a-hustle-free-three-iterator-solution-with-a-grain-of-unsafe/
use std::cmp::Ordering;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let nums1_clone = unsafe { &*(nums1 as *const Vec<i32>) };

        let mut it_nums1 = nums1_clone[..m as usize].iter().rev().peekable();
        let mut it_nums2 = nums2[..n as usize].iter().rev().peekable();

        nums1.iter_mut().rev().for_each(|current| {
            *current = *match it_nums1.peek().cmp(&it_nums2.peek()) {
                Ordering::Less => it_nums2.next(),
                Ordering::Equal => it_nums1.next(),
                Ordering::Greater => it_nums1.next(),
            }
            .unwrap();
        });
    }
}