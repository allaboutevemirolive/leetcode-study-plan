// https://leetcode.com/problems/merge-sorted-array/solutions/2857965/rust-1ms-2-2-memory/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in 0..n {
        nums1[i as usize + m as usize] = nums2[i as usize];
    }
    nums1.sort();
    }
}