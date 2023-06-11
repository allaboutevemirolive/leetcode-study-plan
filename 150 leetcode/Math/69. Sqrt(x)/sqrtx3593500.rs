// https://leetcode.com/problems/sqrtx/solutions/3593500/rust-newton-s-method-approximation/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        fn newton(n: i32, a: i32) -> f64 {
            if n == 0 {
                return 1.0;
            }
            let y = newton(n - 1, a);
            return 0.5 * (y + (a as f64/y));
        }
        let result = newton(20,x);
        result.floor() as i32
    }
}