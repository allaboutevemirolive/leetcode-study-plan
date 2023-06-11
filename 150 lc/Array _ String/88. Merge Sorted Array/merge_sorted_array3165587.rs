// https://leetcode.com/problems/merge-sorted-array/solutions/3165587/rust-solution-using-2-iterators-1ms-2-1mb/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut n2 = nums2.iter();
        nums1
            .iter_mut()
            .skip(m as usize)
            .for_each(|v|
                if *v == 0 {
                    *v = *n2.next().unwrap();
                });
        nums1.sort();
    }
}