// https://leetcode.com/problems/powx-n/solutions/3608731/rust-100-time-and-97-space/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut result = 1.0f64;

        let mut new_base = x;
        let mut new_exp = (n as i64).abs();
        let mut bias = 1.0f64;
        while new_exp > 2 {
            if new_exp % 2 == 0 {
                new_exp = new_exp / 2;
                new_base *= new_base;
            } else {
                bias *= new_base;
                new_exp -= 1;
            }
        }

        for i in 0..new_exp {
            result *= new_base;
        }

        result *= bias;

        if (n > 0) {
            result
        } else {
            1.0f64 / result
        }
    }
}