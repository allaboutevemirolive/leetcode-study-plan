// https://leetcode.com/problems/sqrtx/solutions/2390260/newton-s-method-rust/
fn f(t: f64, x: f64) -> f64 {
    t*t - x
}

fn f_prime(t: f64) -> f64 {
    2.0*t
}

fn newton_iteration(guess: f64, x: f64) -> f64 {
    guess - f(guess, x) / f_prime(guess)
}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let xf = x as f64;
        let mut guess = 1.0;
        
        for _ in 0..100 {
            guess = newton_iteration(guess, xf);
        }
        
        return guess.abs() as i32;
    }
}