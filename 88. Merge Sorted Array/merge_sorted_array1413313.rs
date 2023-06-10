// https://leetcode.com/problems/merge-sorted-array/solutions/1413313/rust-solution/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = (m + n) as usize - 1;
        let mut m = m as usize;
        let mut n = n as usize;

        while i >= 0 && m > 0 && n > 0 {
            if nums1[m - 1] >= nums2[n - 1] {
                nums1[i] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[i] = nums2[n - 1];
                n -= 1;
            }

            i -= 1;
        }

        while n > 0 {
            nums1[i] = nums2[n - 1];
            if n == 0 {
                break;
            }
            n -= 1;
            i -= 1;
        }
    }
}
