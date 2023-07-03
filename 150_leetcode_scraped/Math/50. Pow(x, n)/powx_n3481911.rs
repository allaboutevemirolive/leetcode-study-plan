// https://leetcode.com/problems/powx-n/solutions/3481911/rust-two-solutions/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n < 0 {
            return 1 as f64 / Self::power(x, -(n as i64));
        }

        return Self::power(x, n as i64);
    }

    fn power(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let half = Self::power(x, n / 2);

        if n & 1 == 0 { half * half } else { half * half * x }
    }
}