// https://leetcode.com/problems/merge-sorted-array/solutions/2426690/rust-o-m-n-solution/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut rindex = m + n - 1;

        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums1[rindex] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[rindex] = nums2[n - 1];
                n -= 1;
            }
            rindex -= 1;
        }

        rindex += 1;

        while n != 0 {
            rindex -= 1;
            nums1[rindex] = nums2[n - 1];
            n -= 1;
        }
    }
}