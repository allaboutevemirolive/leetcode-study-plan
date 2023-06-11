// https://leetcode.com/problems/powx-n/solutions/3216380/rust-faster-than-100-bit-maniputlation/
impl Solution {
    pub fn my_pow(mut x: f64,mut n: i32) -> f64 {
        if x == 1.0 || n == 0 {
            return 1.0;
        }
        if x == -1.0 {
            return if n % 2 == 0 {
                1.0
            } else {
                -1.0
            }
        }
        let mut flag = false;
        if n < 0 {
            flag = true;
        }
        if flag && n == i32::MIN {
            return 0.0;
        }
        n = i32::abs(n);
        let mut res = 1.0;
        while n > 0 {
            if n & 1 == 1 {
                res *= x;
            }
            x *= x;
            n >>= 1;
        }
        if flag {
            1.0 / res
        } else {
            res
        }
    }
}