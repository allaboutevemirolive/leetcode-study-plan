// https://leetcode.com/problems/reverse-bits/solutions/3069777/rust-bit-manipulation-0-ms-1-9-mb/
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut reversed: u32 = 0;

        for _ in 0..32 {
            reversed <<= 1;
            reversed |= (1 & x);
            x >>= 1;
        }
        reversed
    }
}