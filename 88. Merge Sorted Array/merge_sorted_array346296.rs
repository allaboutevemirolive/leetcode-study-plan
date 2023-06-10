// https://leetcode.com/problems/merge-sorted-array/solutions/346296/rust-100/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let mut arr = Vec::new();
            nums2.reverse();
            for i1 in 0 .. (m + n) as usize{
                if i1 >= m as usize{
                    arr.push(match nums2.pop() {
                        Some(a) => a,
                        None => 0,
                    });
                }else {
                    arr.push(nums1[i1]);
                }
            }
            arr.sort();
            if  arr.len() < nums1.len() {
                    arr.append(&mut vec![0; nums1.len() - arr.len()]);
            }
            *nums1 = arr;
    }
}