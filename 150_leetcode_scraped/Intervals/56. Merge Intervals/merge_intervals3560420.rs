// https://leetcode.com/problems/merge-intervals/solutions/3560420/rust-clear-solution-using-iterators/
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();
        let mut intervals = intervals.iter_mut();
        let mut curr = intervals.next().unwrap();

        let mut res = Vec::new();
        for interval in intervals {
            if interval[0] <= curr[1] {
                curr[1] = i32::max(curr[1], interval[1])
            } else {
                res.push(curr.to_vec());
                curr = interval;
            }
        }
        res.push(curr.to_vec());
        res
    }
}