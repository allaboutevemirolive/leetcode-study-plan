// https://leetcode.com/problems/climbing-stairs/solutions/3022001/rust-fibonacci-bottom-up-iteration-constant-space/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (2..=n).fold((1,1), |(n_minus_2, n_minus_1), _| (n_minus_1, n_minus_1+n_minus_2)).1
    }
}