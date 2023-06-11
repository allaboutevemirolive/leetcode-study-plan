// https://leetcode.com/problems/climbing-stairs/solutions/3588958/climbing-stairs-using-rust/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut ways = vec![0; (n + 1) as usize];
        ways[1] = 1;
        ways[2] = 2;

        for i in 3..=n as usize {
            ways[i] = ways[i - 1] + ways[i - 2];
        }

        ways[n as usize]
    }
}