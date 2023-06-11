// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2997099/o-n-checking-int-limits-in-c-rust-go/
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut seq: std::collections::HashSet::<i32> = nums.into_iter().collect();
        seq.iter()
            .filter(|&&x| x != std::i32::MIN && !seq.contains(&(x - 1)))
            .map(|&x| (x..=std::i32::MAX).take_while(|x| seq.contains(x)).count())
            .max()
            .unwrap_or(0) as i32
    }
}