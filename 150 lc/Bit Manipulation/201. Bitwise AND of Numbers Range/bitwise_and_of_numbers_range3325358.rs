// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/3325358/0ms-and-2-lines-o-log-left-rust/
impl Solution {
    pub fn range_bitwise_and(l: i32, r: i32) -> i32 {
        if l == 0 { return 0 };
        if l != r { Self::range_bitwise_and(l >> 1, r >> 1) << 1 } else { r }
    }
}