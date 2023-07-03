// https://leetcode.com/problems/insert-interval/solutions/3448520/rust-solution-using-fold/
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut vec = Vec::with_capacity(intervals.len() + 1);
        vec.push(new_interval);
        intervals.into_iter().fold(vec, |mut acc, range| {
            let (start, end) = (range[0], range[1]);
            let last = acc.last_mut().unwrap();
            let (l_start, l_end) = (last[0], last[1]);

            if l_end < start {
                acc.push(range);
            } else if l_start > end {
                acc.insert(acc.len() - 1, range);
            } else {
                last[0] = start.min(l_start);
                last[1] = end.max(l_end);
            }
            acc
        })
    }
}