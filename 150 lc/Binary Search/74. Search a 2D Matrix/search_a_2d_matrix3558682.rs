// https://leetcode.com/problems/search-a-2d-matrix/solutions/3558682/rust-single-binary-search-no-flatten/
use std::cmp::Ordering;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, t: i32) -> bool {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let (mut lbound, mut ubound) = (0, rows*cols);
        let map = |x| ((x/cols, x%cols));
        while lbound < ubound {
            let mid = (lbound+ubound)/2;
            let (i, j) = map(mid);
            match matrix[i][j].cmp(&t) {
                Ordering::Greater => {ubound = mid;}
                Ordering::Less => {lbound = mid+1;}
                Ordering::Equal => return true
            }
        }
        false
    }
}