// https://leetcode.com/problems/merge-sorted-array/solutions/2574715/rust-o-m-n-solution/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, _: i32, nums2: &mut Vec<i32>, _: i32) {
        let (m, n) = (nums1.len(), nums2.len());
        let mut all = Vec::with_capacity(m);
        let (mut a, mut b) = (nums1[..m - n].iter().peekable(), nums2.iter().peekable());
        while let (Some(a0), Some(b0)) = (a.peek(), b.peek()) {
            if a0 <= b0 {
                all.push(**a0);
                a.next();
            } else {
                all.push(**b0);
                b.next();
            }
        }
        all.extend(a);
        all.extend(b);
        *nums1 = all;
    }
}