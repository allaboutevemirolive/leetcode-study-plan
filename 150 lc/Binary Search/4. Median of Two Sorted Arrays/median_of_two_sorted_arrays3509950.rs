// https://leetcode.com/problems/median-of-two-sorted-arrays/solutions/3509950/fast-and-space-efficient-rust-solution/
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut l = 0;
        let mut r = 0;
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut i = 0;
        let concat_len = nums1.len() + nums2.len();
        let half_len = concat_len / 2;
        let mut concat = vec![0 as i32; half_len + 1];

        const INF: i32 = 2147483647;

        loop {
            let lv = if l < len1 { nums1[l] } else { INF };
            let rv = if r < len2 { nums2[r] } else { INF };
            if lv <= rv {
                l += 1;
                concat[i] = lv;
            } else {
                r += 1;
                concat[i] = rv;
            }
            i += 1;
            if i == half_len + 1 {
                if concat_len % 2 == 0 {
                    let m1 = concat[half_len];
                    let m2 = concat[(half_len) - 1];
                    return ((m1 as f64 + m2 as f64) / 2.0) as f64
                }
                else {
                    return concat[half_len] as f64
                }
            }
        }

        return 0.
    }
}