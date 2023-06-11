// https://leetcode.com/problems/merge-sorted-array/solutions/343706/rust-solution-0ms/
// https://github.com/wangyuntao/leetcode-rs
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let l1 = m as usize;
        let l2 = n as usize;
        assert!(nums1.len() >= l1 + l2);
        (&mut nums1[l1..l1 + l2]).copy_from_slice(nums2);

        for i in l1..l1 + l2 {
            let mut j = i;
            while j > 0 {
                if nums1[j] < nums1[j - 1] {
                    nums1.swap(j, j - 1);
                    j -= 1;
                    continue;
                }
                if j == i {
                    return;
                }
                break;
            }
        }
    }
}