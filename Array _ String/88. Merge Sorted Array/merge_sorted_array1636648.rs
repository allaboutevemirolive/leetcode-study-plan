// https://leetcode.com/problems/merge-sorted-array/solutions/1636648/rust-for-each/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m: usize = m as usize;  // m now is a wall betwean zeros and numbers
        let mut i: usize = 0;
        nums2.iter().for_each(|n2| {
            while i < nums1.len() {
                if *n2 < nums1[i] || i == m {  // if we recive the wall then insert rest of the nums2
                    nums1.insert(i, *n2);
                    nums1.pop();  // remove the stupid trailing zeroes filling nums1
                    i += 1;
                    m += 1;  // increase the wall
                    break;
                } 
                i += 1;
            }
        });
    }
}