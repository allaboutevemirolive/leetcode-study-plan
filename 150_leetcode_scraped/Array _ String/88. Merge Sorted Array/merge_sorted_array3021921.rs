// https://leetcode.com/problems/merge-sorted-array/solutions/3021921/learning-rust-merge-sorted-array/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut pointer = (m + n) as usize;
        let mut m_idx = m as usize; 
        let mut n_idx = n as usize;

        nums1.insert(0, i32::MIN);
        nums2.insert(0, i32::MIN);

        while pointer > 0 {
            let m_val = nums1[m_idx];
            let n_val = nums2[n_idx];

            if m_val < n_val {
                nums1[pointer] = n_val;
                n_idx -= 1; 
            
            } else {
                nums1[pointer] = m_val;
                m_idx -= 1;
            }
            pointer -= 1;
        }
        nums1.retain(|&x| x != i32::MIN);
    }
}