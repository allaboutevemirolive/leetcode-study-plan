// https://leetcode.com/problems/powx-n/solutions/2709695/d-c-solution-in-rust/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn helper(x: f64, n: i32) -> f64 {
            if x == 0.0 { return 0.0 }
            if n == 0 { return 1.0 }

            let arg_x = x.powf(2.0);
            let arg_n = n / 2;
            let result = helper(arg_x, arg_n);
            if n % 2 == 0 { result }
            else { x * result }
        }

        let result = helper(x, n.abs());
        if n >= 0 { result }
        else { 1.0 / result }
    }
}