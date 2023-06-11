// https://leetcode.com/problems/median-of-two-sorted-arrays/solutions/3360786/rust/
impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();

        let (target, extra) = if (n + m) % 2 == 0 {
            ((n + m) / 2 - 1, true)
        } else {
            ((n + m) / 2, false)
        };

        // tail guard, save us from index checking
        nums1.push(1_000_001);
        nums2.push(1_000_001);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut cur = 0;
        loop {
            cur = if nums1[i] < nums2[j] {
                i += 1;
                nums1[i - 1]
            } else {
                j += 1;
                nums2[j - 1]
            };

            if k == target {
                if extra {
                    if nums1[i] < nums2[j] {
                        return (cur + nums1[i]) as f64 / 2.;
                    } else {
                        return (cur + nums2[j]) as f64 / 2.;
                    }
                } else {
                    return cur as f64;
                }
            }
            k += 1;
        }
    }
}