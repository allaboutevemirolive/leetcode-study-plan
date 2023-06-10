// https://leetcode.com/problems/merge-sorted-array/solutions/298468/rust-solution/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m1 = nums1.len();
        let m = m as usize;
        let n = n as usize;
        
        for i in (0..m).rev() {
            nums1.swap(i, m1 - m + i);
        }
        
        let mut i = m1 - m;
        let mut j = 0;
        let mut k = 0;
        
        while i < m1 || j < n {
            if (j == n || (i < m1 && nums1[i] < nums2[j])) {
                nums1.swap(k, i);
                i += 1;
            } else {
                nums1[k] = nums2[j];
                j += 1;
            }
            k += 1;
        }
    }
}