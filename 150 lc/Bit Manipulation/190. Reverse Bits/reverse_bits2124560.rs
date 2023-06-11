// https://leetcode.com/problems/reverse-bits/solutions/2124560/rust-simulate-o-n/
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let n = 31;
        let mut ans = 0;
        for i in 0..32 {
            let bit = (x >> i) & 1;
            let v = bit << (n-i);
            ans += v;
        }
        ans
    }
}