// https://leetcode.com/problems/reverse-bits/solutions/3032791/rust-bit-shift/
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let (mut reversed, mut power) : (u32, u32) = (0,31);
        while x > 0 {
            reversed += (x & 1) << power;
            x >>= 1;
            power -= 1;
        }
        reversed
    }
}