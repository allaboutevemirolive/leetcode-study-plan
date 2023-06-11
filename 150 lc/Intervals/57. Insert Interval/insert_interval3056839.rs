// https://leetcode.com/problems/insert-interval/solutions/3056839/rust-easy-o-n-solution-3ms/
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut s, mut e) = (new_interval[0], new_interval[1]);
        let mut left: Vec<Vec<i32>> = Vec::new();
        let mut right: Vec<Vec<i32>> = Vec::new();
        for i in intervals.into_iter() {
            if i[1] < s {
                left.push(i.to_vec());
            } else if i[0] > e {
                right.push(i.to_vec());
            } else {
                s = s.min(i[0]);
                e = e.max(i[1]);
            }
        }
        [left, vec![vec![s, e]], right].concat()
    }
}