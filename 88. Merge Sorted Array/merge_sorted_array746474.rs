// https://leetcode.com/problems/merge-sorted-array/solutions/746474/rust-cheapest-best/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        for i in (0..nums1.len()).rev() {
            let a = nums1.get((m - 1) as usize).unwrap_or(&std::i32::MIN);
            let b = nums2.get((n - 1) as usize).unwrap_or(&std::i32::MIN);
            match a.cmp(b) {
                std::cmp::Ordering::Greater => { nums1[i] = *a; m -= 1 },
                _ => { nums1[i] = *b; n -= 1 }
            }
        }
    }
}