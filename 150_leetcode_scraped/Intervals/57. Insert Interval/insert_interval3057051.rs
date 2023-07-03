// https://leetcode.com/problems/insert-interval/solutions/3057051/rust-easy-to-understand-solution/
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n = intervals.len();
        let mut i = 0;
        let mut ans = vec![];

        while i < n && intervals[i][1] < new_interval[0] {
            ans.push(intervals[i].clone());
            i += 1;
        }

        let (mut start, mut end) = (new_interval[0], new_interval[1]);

        while i < n && intervals[i][0] <= new_interval[1] {
            start = std::cmp::min(start, intervals[i][0]);
            end = std::cmp::max(end, intervals[i][1]);
            i += 1;
        }
        ans.push(vec![start, end]);

        while i < n {
            ans.push(intervals[i].clone());
            i += 1;
        }

        ans
    }
}