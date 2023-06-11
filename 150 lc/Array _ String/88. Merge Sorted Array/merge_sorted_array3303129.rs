// https://leetcode.com/problems/merge-sorted-array/solutions/3303129/rust-beats-100/
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < m && j < n {
        let val1 = nums1[i as usize];
        let val2 = nums2[j as usize];
        result.push(match val1.cmp(&val2) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {
                i += 1;
                val1
            }
            std::cmp::Ordering::Greater => {
                j += 1;
                val2
            }
        });
    }
    if i < m {
        result.extend_from_slice(&nums1[(i as usize)..(m as usize)]);
    }
    if j < n {
        result.extend_from_slice(&nums2[(j as usize)..(n as usize)]);
    }
    *nums1 = result;
}