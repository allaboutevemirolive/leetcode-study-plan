// https://leetcode.com/problems/number-of-1-bits/solutions/3098487/rust-multiple-solutions-beats-100/
impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut count = 0;
        while n > 0 {
            count += (n % 2) as i32;
            n = n / 2;
        }
        count
    }
}