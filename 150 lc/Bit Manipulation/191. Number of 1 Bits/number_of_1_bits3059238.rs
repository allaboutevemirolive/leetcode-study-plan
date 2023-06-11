// https://leetcode.com/problems/number-of-1-bits/solutions/3059238/rust-iteration-0-ms-2-1-mb/
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        // n.count_ones() as i32
        let mut count = 0;
        format!("{n:b}").into_bytes().into_iter().for_each(|b| {
            count += (b as i32 - 48);
        });
        count
    }
}