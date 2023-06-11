// https://leetcode.com/problems/merge-intervals/solutions/3227849/rust-easy-solution/
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a,b| a[0].cmp(&b[0]));

        let mut ans : Vec<Vec<i32>> = vec![];
        let mut start : i32 = intervals[0][0];
        let mut end : i32 = intervals[0][1];

        for x in 1..intervals.len() {
            if end >= intervals[x][0] {
                end = std::cmp::max(intervals[x][1],end);
            }
            else {
                ans.push([start,end].to_vec());
                start = intervals[x][0];
                end = intervals[x][1];
            }
        }
        ans.push([start,end].to_vec());
        ans
    }
}