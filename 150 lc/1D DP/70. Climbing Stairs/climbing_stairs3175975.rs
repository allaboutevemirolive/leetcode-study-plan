// https://leetcode.com/problems/climbing-stairs/solutions/3175975/rust-fast-o-n-solution/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..=n)
            .fold((1, 0), |(prev1, prev2), _| (prev2, prev1 + prev2))
            .1
    }
}