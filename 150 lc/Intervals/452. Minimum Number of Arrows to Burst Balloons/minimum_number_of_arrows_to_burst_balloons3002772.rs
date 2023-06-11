// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3002772/rust-checking-overlap-while-popping-45ms/
use std::cmp::Reverse;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        let mut count  = 0;
        points.sort_unstable_by_key(|k| Reverse(k[1]));

        while let Some(mut ol_range) = points.pop().map(|p| [p[0], p[1]]) {
            while points.last()
                        .map(|p| is_overlapped(p, &ol_range))
                        .unwrap_or(false) {
                ol_range = overlap(&points.pop().unwrap(), &ol_range);
            }
            count += 1;
        }
        count
    }
}
#[inline]
fn is_overlapped(a: &[i32], b: &[i32]) -> bool {
    a[0] <= b[1] && a[1] >= b[0]
}
#[inline]
fn overlap(a: &[i32], b: &[i32]) -> [i32; 2] {
    [a[0].max(b[0]), a[1].min(b[1])]
}