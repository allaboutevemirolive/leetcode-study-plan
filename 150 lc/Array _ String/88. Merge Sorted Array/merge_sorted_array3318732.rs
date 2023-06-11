// https://leetcode.com/problems/merge-sorted-array/solutions/3318732/rust-merge-from-end-block-copy/
use std::cmp::Ordering;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        loop {
            match (m.cmp(&0), n.cmp(&0)) {
                (Ordering::Greater, Ordering::Greater) => {
                    nums1[m+n] = match nums1[m-1].cmp(&nums2[n-1]){
                        Ordering::Greater => {
                            m -= 1;
                            nums1[m]
                        }, 
                        _ => {
                            n -= 1;
                            nums2[n]
                        }
                    };
                }, 
                (Ordering::Equal, Ordering::Greater) => {
                    nums1[..n].copy_from_slice(&nums2[..n]);
                    break;
                },
                _ => break,
            };
        }
    }
}