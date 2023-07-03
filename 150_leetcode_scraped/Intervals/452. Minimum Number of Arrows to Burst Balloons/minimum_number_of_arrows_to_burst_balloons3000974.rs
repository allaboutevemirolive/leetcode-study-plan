// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3000974/rust-runtime-pattern-matching-has-noticeable-performance-overhead/
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();
        let mut ans = 0;
        let mut i = i32::MIN;
        for pt in points {
            if let [a, z] = pt[..] {
                if a > i {
                    ans += 1;
                    i = z;
                } else {
                    i = i.min(z);
                }
            }
        }
        ans.max(1)
    }
}