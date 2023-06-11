// https://leetcode.com/problems/rotate-image/solutions/2680102/intuitive-rust-solution-transpose-reverse/
Intuitive solution: 

1 2 3   transpose    1 4 7  reverse elements in rows   7 4 1 
4 5 6   ---------->  2 5 8  ------------------------>  8 5 2
7 8 9                3 6 9                             9 6 3
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        /// Rotate 90 degrees clockwise = matrix transpose -> reverse each row elements
        let n = matrix.len();
        
        // Transpose the matrix
        for i in (0..n) {
            for j in (i..n) {
                let temp1 = matrix[j][i];
                let temp2 = std::mem::replace(&mut matrix[i][j], temp1);
                std::mem::replace(&mut matrix[j][i], temp2);
                // unsafe {
                //     std::ptr::swap(&mut matrix[j][i] as *mut i32, &mut matrix[i][j] as *mut i32);
                // }
            }
        }
        
        // Reverse the elements of every row
        for i in (0..n) {
            for j in (0..(n/2)) {
                let temp1 = matrix[i][j];
                let temp2 = std::mem::replace(&mut matrix[i][n-1-j], temp1);
                std::mem::replace(&mut matrix[i][j], temp2);
                // unsafe {
                //     std::ptr::swap(&mut matrix[i][j] as *mut i32, &mut matrix[i][n-1-j] as *mut i32);   
                // }
            }
        }
    }