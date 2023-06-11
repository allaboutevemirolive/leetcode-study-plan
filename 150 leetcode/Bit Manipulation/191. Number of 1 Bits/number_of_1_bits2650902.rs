// https://leetcode.com/problems/number-of-1-bits/solutions/2650902/rust-count-ones-one-word-solution/
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        n.count_ones() as i32
    }
}