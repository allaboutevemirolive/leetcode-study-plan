// https://leetcode.com/problems/reverse-bits/solutions/2975541/rust-simple-o-1-2-solutions/
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        //x.reverse_bits()

        x =               (x >> 16) | (x << 16);
        x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
        x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
        x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
            ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1)
        
    }
}