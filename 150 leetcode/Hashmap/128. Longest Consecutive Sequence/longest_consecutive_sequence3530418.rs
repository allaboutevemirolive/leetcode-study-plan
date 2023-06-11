// https://leetcode.com/problems/longest-consecutive-sequence/solutions/3530418/minimal-rust-o-n-solution/
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let nums: HashSet<_> = nums.into_iter().collect();

        nums.iter()
            .filter(|n| !nums.contains(&(*n - 1)))
            .map(|n| (*n..).take_while(|next| nums.contains(next)).count())
            .max()
            .unwrap_or_default() as i32
    }
}
