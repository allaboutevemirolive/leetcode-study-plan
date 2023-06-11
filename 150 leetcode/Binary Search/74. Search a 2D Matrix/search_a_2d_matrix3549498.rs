// https://leetcode.com/problems/search-a-2d-matrix/solutions/3549498/o-log-m-n-rust-solution/
impl Solution {
    #[inline(always)]
    fn get_row_and_idx(total_idx: usize, row_length: usize) -> (usize, usize) {
        let row = total_idx / row_length;
        let idx = total_idx % row_length;
        (row, idx)
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let total_elements = matrix.len() * matrix[0].len();
        let (mut start, mut end) = (0, total_elements);
        while start < end {
            let mid = start + (end - start) / 2;
            let (row, idx) = Self::get_row_and_idx(mid, matrix[0].len());
            if target > matrix[row][idx] {
                start = mid + 1;
            } else if target < matrix[row][idx] {
                end = mid;
            } else {
                return true;
            }
        }
        false
    }
}
