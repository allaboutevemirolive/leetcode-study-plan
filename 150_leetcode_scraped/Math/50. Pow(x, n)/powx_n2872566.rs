// https://leetcode.com/problems/powx-n/solutions/2872566/solution-in-rust-0-ms/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res = 1.0;
        let sgn = n > 0;
        let mut n = (n as i64).abs(); // convert to i64 to handle the i32::MIN case
        let mut x = x;
        
		// This is a fast way to implement binary exponentiation. https://cp-algorithms.com/algebra/binary-exp.html
        while (n > 0) {
            if (n & 1) > 0 {
                res *= x;
            }
            x = x * x;
            n >>= 1;
        }
        
        if !sgn {
            res = 1.0 / res;
        }
        
        res
    }
}