// https://leetcode.com/problems/climbing-stairs/solutions/3117373/rust-geeks/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut cur, mut prev) = (1, 0);
        for _ in 0..n {
            std::mem::swap(&mut cur, &mut prev);
            cur += prev;
        }
        cur
    }
}