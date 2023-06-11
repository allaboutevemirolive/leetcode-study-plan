// https://leetcode.com/problems/search-a-2d-matrix/solutions/1895810/rust-matrix-traversal-o-n-explained/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // Matrix boundaries matrix[0 <= y < m][0 <= x < n]
        let (m, n) = (matrix.len(), matrix[0].len());
        // Start at top left of matrix
        let (mut y, mut x) = (0, 0);
        loop {
            // Each itteration we will either: return answer OR go down OR go right
            if matrix[y][x] == target {
                // we found our target
                return true;
            } else if y + 1 < m && matrix[y + 1][x] <= target {
                // if element down is smaller or equal to target, we should go down
                y += 1;
            } else if x + 1 < n && matrix[y][x + 1] <= target {
                // if element right is smaller or equal to target, we should go right
                x += 1;
            } else {
                // if there is no valid choice we are done
                return false;
            }
        }
    }
}