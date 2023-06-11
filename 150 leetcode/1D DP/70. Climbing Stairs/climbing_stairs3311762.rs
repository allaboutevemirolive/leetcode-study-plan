// https://leetcode.com/problems/climbing-stairs/solutions/3311762/rust-100-solution/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 2;

        for _ in 1..n {
            let temp = b;
            b = b + a;
            a = temp;
        }

        return a;
    }
}