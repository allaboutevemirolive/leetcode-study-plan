// https://leetcode.com/problems/merge-sorted-array/solutions/1176177/rust-0ms/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums1_pos: i32 = m - 1;
        let mut nums2_pos: i32 = n - 1;
        
        // If nums2 is empty, nums1 does not need modifying
        if (nums2.len() == 0) {
            return;
        }

        for i in (0..nums1.len()).rev() {
            if (
                nums2_pos < 0 || // If we are at the end of nums2, we definitely want nums1
                (nums1_pos >= 0 && // If we are at the end of nums1, we definitely want nums2
					nums1[nums1_pos as usize] >= nums2[nums2_pos as usize]) // We have a valid comparison to make, choose the larger number
            ) { // Set nums[i] to the greater number and decrement the position of the Vec whose number we picked
                nums1[i] = nums1[nums1_pos as usize];
                nums1_pos -= 1;
            } else {
                nums1[i] = nums2[nums2_pos as usize];
                nums2_pos -= 1;
            }
        }
    }
}