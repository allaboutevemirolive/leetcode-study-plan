// https://leetcode.com/problems/rotate-image/solutions/2637965/rust-2ms-solution-2-1-mb-memory/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        //First step is to invert the rows: if we have a 3x3 matrix as 
		//[row1, row2, row3] we transform it to [row3, row2, row1]
        let mut temp: Vec<i32> = Vec::new();
        let matrix_length = matrix.len();

        for i in 0..(matrix_length / 2) {
            temp = matrix[i].clone();
            matrix[i] = matrix[matrix_length - i - 1].clone();
            matrix[matrix_length - i - 1] = temp;
        }

        //We wan now swap the digits along the diagonal
        let mut temp_diag: i32 = 0;

        for i in 0..matrix.len() {
            for j in i..matrix[0].len() {
                temp_diag = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp_diag;
            }
        }
    }
}  