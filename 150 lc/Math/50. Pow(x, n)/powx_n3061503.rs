// https://leetcode.com/problems/powx-n/solutions/3061503/simple-rust-non-recursive-binary-solution/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res = 1.0;
        let mut multiplier = if n > 0 { x } else { 1.0 / x };
        let mut left = (n as i64).abs();
        while left > 0 {
            if left & 1 == 1 {
                res *= multiplier;
            }
            multiplier *= multiplier;
            left >>= 1;
        }
        res
    }
}