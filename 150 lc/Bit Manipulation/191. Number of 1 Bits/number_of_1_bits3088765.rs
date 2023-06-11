// https://leetcode.com/problems/number-of-1-bits/solutions/3088765/rust/
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut n = n;
        let mut count = 0;
        loop {
           if n & 1 != 0 {
             count += 1;            
           }
           n = n >> 1;
           if n == 0 {
               break
           }
        }
        count
    }
}