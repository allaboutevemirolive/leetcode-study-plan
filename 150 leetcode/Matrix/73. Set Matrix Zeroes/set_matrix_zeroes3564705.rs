// https://leetcode.com/problems/set-matrix-zeroes/solutions/3564705/simple-and-easy-solution-rust-noob/
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut zero_position: Vec<Vec<usize>> = vec![]; 
        for i in 0..matrix.len(){
            for j in 0..matrix[0].len(){
                if matrix[i][j] == 0{
                    zero_position.push(vec![i,j]);
                }
            }
        }   
        // println!("{:?}", zero_position);
        for i in 0..zero_position.len(){
            println!("{:?}", zero_position[i]);
            // row below
            let mut row = zero_position[i][0];
            let mut col = zero_position[i][1];
            while row < matrix.len(){
                matrix[row][col] = 0;
                row += 1;
            }
            // row above
            let mut row = zero_position[i][0];
            let mut col = zero_position[i][1];
            while row >= 0{
                matrix[row][col] = 0;
                if row == 0 {break}
                row -= 1;
            }
            // col right
            let mut row = zero_position[i][0];
            let mut col = zero_position[i][1];
            while col < matrix[0].len(){
                matrix[row][col] = 0;
                col += 1;
            }
            // col left
            let mut row = zero_position[i][0];
            let mut col = zero_position[i][1];
            while col >= 0{
                matrix[row][col] = 0;
                if col == 0{break}
                col -= 1;
            }
        }
    }
}