// https://leetcode.com/problems/merge-sorted-array/solutions/2010261/rust-o-n-m-iterators/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // Iterate over vectors in reverse and pick numbers in descending
        // order, since we use the available space at the end of nums1.
        let mut i = (m as usize).wrapping_sub(1);
        let mut j = (n as usize).wrapping_sub(1);
        let mut k = ((m+n) as usize).wrapping_sub(1);

        // As long as there are numbers left in both nums1 and nums2,
        // we pick the highest between them as the next entry in the
        // merged result.
        while i < m as usize && j < n as usize {
            if nums1[i] > nums2[j] {
                nums1[k] = nums1[i];
                i = i.wrapping_sub(1);
            } else {
                nums1[k] = nums2[j];
                j = j.wrapping_sub(1);
            }
            k = k.wrapping_sub(1);
        }

        // If there are numbers left in nums1, they are already sorted and in
        // place. If nums2 is not exhausted, those numbers can simply be copied
        // to the corresponding indices in the result.
        if j < n as usize {
            for i in 0..=j {
                nums1[i] = nums2[i];
            }
        }
    }
}