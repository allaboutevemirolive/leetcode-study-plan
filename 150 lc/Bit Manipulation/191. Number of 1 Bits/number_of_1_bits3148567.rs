// https://leetcode.com/problems/number-of-1-bits/solutions/3148567/the-fastest-rust-solution-100/
impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut count = 0;

        for _ in 0..32 {
            count += n & 1;
            n >>= 1;
        }
        
        count as _
    }
}