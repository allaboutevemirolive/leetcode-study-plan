// https://leetcode.com/problems/merge-sorted-array/solutions/315821/rust-implement/
 pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) -> Vec<i32> {
        nums1.truncate(m as usize);
        nums1.append(nums2);
        nums1.sort();
        nums1.to_vec()
    }