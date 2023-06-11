// https://leetcode.com/problems/rotate-image/solutions/2506446/rust-solution-0-ms/
 pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
           let l = matrix.len();

    // transpose

    let mut temp = 0;
    for i in 0..l {
        for j in i..l {
            temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    // reverse

    for i in 0..l {
        for j in 0..l / 2 {
            temp = matrix[i][j];
            matrix[i][j] = matrix[i][l - 1 - j];
            matrix[i][l - 1 - j] = temp;
        }
    }
    }