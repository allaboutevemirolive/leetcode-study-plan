// https://leetcode.com/problems/sqrtx/solutions/3016045/rust-brute-force-search/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // catching integer overflow case
        if (x >= 2147395600) {
            return 46340
        }
        let mut guess: i32 = 1;
        while !(guess * guess > x) {
            guess = guess + 1;
        }
        guess - 1
    }
}