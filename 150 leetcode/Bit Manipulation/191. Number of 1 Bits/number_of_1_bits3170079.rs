// https://leetcode.com/problems/number-of-1-bits/solutions/3170079/rust-flip-least-significant-one-bit/
impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut num_ones = 0;
        loop {
            if n == 0 {
                return num_ones;
            }
            n &= n-1;
            num_ones +=1;
        }
        unreachable!();
    }
}