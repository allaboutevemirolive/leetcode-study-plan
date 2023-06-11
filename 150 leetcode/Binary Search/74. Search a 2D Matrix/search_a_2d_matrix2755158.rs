// https://leetcode.com/problems/search-a-2d-matrix/solutions/2755158/binary-search-rust-solution/
use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let (mut lo, mut hi) = (0, rows*cols);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            let row = mid / cols;
            let col = mid % cols;

            match matrix[row][col].cmp(&target) {
                Ordering::Less => lo = mid + 1,
                Ordering::Equal => return true,
                Ordering::Greater => hi = mid,
            }
        }
        false
    }
}