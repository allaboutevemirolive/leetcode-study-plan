// https://leetcode.com/problems/merge-intervals/solutions/3493771/rust-solution-with-optimized-time-space-complexity-59-77/
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|a| a[0]);
        let mut result = Vec::with_capacity(intervals.len());
        let mut current_interval = intervals[0].clone();
        for interval in intervals.into_iter() {
            if current_interval[1] >= interval[0] {
                current_interval[1] = current_interval[1].max(interval[1]);
            } else {
                result.push(current_interval);
                current_interval = interval;
            }
        }
        result.push(current_interval);
        result

    }
}