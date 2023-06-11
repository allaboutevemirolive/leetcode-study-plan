// https://leetcode.com/problems/reverse-bits/solutions/2996763/rust-clean-iterator-fold-0-ms-100/
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        (0..32).fold(0u32, |mut result, _| {
            result = (result << 1) | (x & 1);
            x >>= 1;
            result
        })
    }
}