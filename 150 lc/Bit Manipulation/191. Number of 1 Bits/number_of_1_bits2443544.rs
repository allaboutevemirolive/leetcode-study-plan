// https://leetcode.com/problems/number-of-1-bits/solutions/2443544/0ms-rust-solution/
impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut ans = 0;

        while n != 0 {
            ans += n & 1;
            n >>=1;
        }

        ans as i32
    }
}