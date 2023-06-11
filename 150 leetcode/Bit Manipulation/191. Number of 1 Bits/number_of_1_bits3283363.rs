// https://leetcode.com/problems/number-of-1-bits/solutions/3283363/rust-solution/
impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut ans = n & 1;
        while n > 0 {
            ans += (n >> 1) & 1;
            n >>= 1;
        }
        ans as i32
    }
}