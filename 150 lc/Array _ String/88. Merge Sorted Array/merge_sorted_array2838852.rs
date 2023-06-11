// https://leetcode.com/problems/merge-sorted-array/solutions/2838852/rust-o-n-simple-efficient-0ms/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;
        let mut k = i + j;

        while i < k {
            if i > 0 && nums1[i - 1] > nums2[j - 1] {
                nums1[k - 1] = nums1[i - 1];
                i -= 1;
            }
            else {
                nums1[k - 1] = nums2[j - 1];
                j -= 1;
            }
            k -= 1;
        }
    }
}