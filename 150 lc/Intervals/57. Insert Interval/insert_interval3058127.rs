// https://leetcode.com/problems/insert-interval/solutions/3058127/go-rust-csharp-binary-search-splice/
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        match match (
            intervals.binary_search_by(|v| v[1].cmp(&new_interval[0])),
            intervals.binary_search_by(|v| v[0].cmp(&new_interval[1])),
        ) {
            (Err(start), Err(end)) | (Ok(start), Err(end)) => (start, end),
            (Err(start), Ok(end)) | (Ok(start), Ok(end)) => (start, end + 1),
        } {
            (start, end) => match match start != end {
                true => vec![
                    new_interval[0].min(intervals[start][0]),
                    new_interval[1].max(intervals[end - 1][1]),
                ],
                false => new_interval,
            } {
                r => intervals.spliced(start..end, vec![r]),
            },
        }
    }
}

use std::ops::RangeBounds;
trait Monad {
    fn spliced(self, range: impl RangeBounds<usize>, replace_with: Self) -> Self;
}
impl Monad for Vec<Vec<i32>> {
    fn spliced(mut self, range: impl RangeBounds<usize>, replace_with: Self) -> Self {
        self.splice(range, replace_with);
        self
    }
}