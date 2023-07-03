// https://leetcode.com/problems/reverse-bits/solutions/2517477/rust-c-like-approach/
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut res = 0;

        for i in 0..32 {
            res = res << 1;
            res |= (x >> i) & 1;
        }

        res
    }
}