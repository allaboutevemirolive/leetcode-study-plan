// https://leetcode.com/problems/merge-intervals/solutions/2816556/rust-solution-o-nlogn/
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::{min, max};
        fn is_overlap(i1: &[i32], i2: &[i32]) -> bool {
            i1[1] >= i2[0]
        }
        intervals.sort_by(|i1, i2| i1[0].cmp(&i2[0]));

        let mut res = vec![intervals.remove(0)];

        for i2 in &intervals {
            let i1 = &res[res.len() - 1];
            if is_overlap(i1, i2) {
                let to_push = vec![min(i1[0], i2[0]), max(i1[1], i2[1])];
                res.pop();
                res.push(to_push);
            } else {
                res.push(i2.clone());
            }
        }
        res
    }
}