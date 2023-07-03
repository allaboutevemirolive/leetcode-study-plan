// https://leetcode.com/problems/merge-intervals/solutions/3576221/learning-rust-use-iterators-don-t-index/
use std::cmp::max;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals_copy = intervals.clone(); 
        intervals_copy.sort();

        let mut ans = vec![intervals_copy[0].clone()];

        for interval in intervals_copy.iter() {
            let last = ans.len() - 1;
            let prev_y = ans[last][1];
            let (curr_x, curr_y) = (interval[0], interval[1]);

            if prev_y >= curr_x {
                ans[last][1] = max(prev_y, curr_y);
            } else {
                ans.push(vec![curr_x, curr_y]);
            }
        }
        ans
    }
}