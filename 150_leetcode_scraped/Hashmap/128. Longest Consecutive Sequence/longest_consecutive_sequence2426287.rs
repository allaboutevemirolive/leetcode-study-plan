// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2426287/idiomatic-rust-code-written-with-the-help-of-copilot/
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // hashset containing all the numbers in the input
        let numset = nums.into_iter().collect::<HashSet<i32>>();

        let mut visited = HashSet::<i32>::new();

        let mut max_len = 0;

        for &n in &numset{
            // if we have already visited this number, skip it
            if visited.contains(&n){
                continue;
            }
            visited.insert(n);
            let mut rangelen = 1;
            // count up
            (n+1..).take_while(|&tmp| numset.contains(&tmp))
                   .for_each(|tmp| {
                        rangelen += 1;
                        visited.insert(tmp);
                   });
            // count down
            (1..).map(|i| n - i)
                 .take_while(|&tmp| numset.contains(&tmp))
                 .for_each(|tmp| {
                        rangelen += 1;
                        visited.insert(tmp);
                });
            max_len = std::cmp::max(max_len, rangelen);
        }
        
        max_len
    }
}