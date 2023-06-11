// https://leetcode.com/problems/powx-n/solutions/2559484/rust-simple-iterative-and-recursive-solution/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut ans:f64 = 1.0;
        let mut x = x;
        let mut n = n as i64;
        
        if n < 0 {
            x = 1.0/x;
            n = -n;
        }
        
        while n > 0 {
           if n % 2 != 0 {
               ans *= x;
           }
           x = x*x;
           n /= 2;
        }
        
        ans
        
    }
}