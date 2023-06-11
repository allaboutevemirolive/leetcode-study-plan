// https://leetcode.com/problems/merge-sorted-array/solutions/1979529/rust-0ms-4-lines-with-sort-unstable/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in 0..n {
            nums1[(i+m) as usize]=nums2[i as usize];
        }
        return nums1.sort_unstable();
    }
}