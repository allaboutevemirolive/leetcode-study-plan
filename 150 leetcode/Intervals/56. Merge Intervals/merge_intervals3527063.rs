// https://leetcode.com/problems/merge-intervals/solutions/3527063/merge-interval-rust/
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
          // Sort intervals by the start value
        intervals.sort_by_key(|interval| interval[0]);

        let mut merged_intervals: Vec<Vec<i32>> = Vec::new();
        let mut current_interval: Vec<i32> = intervals[0].clone();

        for interval in intervals.into_iter().skip(1) {
            let start = interval[0];
            let end = interval[1];

            if start <= current_interval[1] {
                current_interval[1] = current_interval[1].max(end);
            } else {
                merged_intervals.push(current_interval);
                current_interval = interval.clone();
            }
        }

        merged_intervals.push(current_interval);

        merged_intervals
    }
}