// https://leetcode.com/problems/reverse-bits/solutions/3283801/rust-solution/
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        // x.reverse_bits() cheat lol
        let mut ans = 0u32;
        for i in 0..=31 {
            let bit_left = x >> i & 1;
            let bit_pos = 31 - i;
            ans |= bit_left << (bit_pos);
        }
        ans
    }
}