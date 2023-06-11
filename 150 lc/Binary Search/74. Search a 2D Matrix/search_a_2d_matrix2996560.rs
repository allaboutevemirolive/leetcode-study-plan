// https://leetcode.com/problems/search-a-2d-matrix/solutions/2996560/rust-binary-search-and-linear-search-solutions/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let mut start = 0;
        let mut end = m * n - 1;

        while start <= end {
            let mid = start + (end - start) / 2;
            let i = (mid / n) as usize;
            let j = (mid % n) as usize;

            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] < target {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }

        false
    }