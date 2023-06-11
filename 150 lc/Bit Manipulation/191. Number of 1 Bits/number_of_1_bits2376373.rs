// https://leetcode.com/problems/number-of-1-bits/solutions/2376373/very-simple-rust-solution-0ms-faster-than-100/
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut n = n;

        let mut num: i32 = 0;
        while n > 0 {
            num += (n & 1) as i32;
            n >>= 1;
        }

        return num;
    }
}
