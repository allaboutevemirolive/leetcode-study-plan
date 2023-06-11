// https://leetcode.com/problems/merge-sorted-array/solutions/240738/rust-0ms-2-5mb/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in m..m + n {
            nums1[i as usize] = nums2[(i - m) as usize];
        }
        let l = nums1.len();
        nums1.truncate((m + n) as usize);
        nums1.sort();
        for _i in m + n..l as i32 {
            nums1.push(0);
        }
    }
}