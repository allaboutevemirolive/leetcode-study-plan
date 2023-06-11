// https://leetcode.com/problems/powx-n/solutions/2099143/rust-recursive-and-iterative-solutions/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1f64;
        }
        
        let mut x = x;
        let mut n: i64 = n as i64;
        
        if n < 0 {
            x = 1f64 / x;
            n = -n;
        }
        
        let mut res: f64 = 1f64;
        
        while n > 0 {
            if n & 1 == 1 {
                res = res * x;
            }
            
            x = x * x;
            n >>= 1;
        }
        
        res
    }
}