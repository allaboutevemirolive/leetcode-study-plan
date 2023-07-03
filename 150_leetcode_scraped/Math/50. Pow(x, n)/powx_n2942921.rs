// https://leetcode.com/problems/powx-n/solutions/2942921/solution-for-pow-x-n-in-rust/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // Implement the built-in `f64::powf` function
        // And convert `n` to `f64` using `f64::from()`
        f64::powf(x, f64::from(n))
    }
}