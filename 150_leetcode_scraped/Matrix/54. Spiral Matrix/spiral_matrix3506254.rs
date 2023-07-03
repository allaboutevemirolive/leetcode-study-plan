// https://leetcode.com/problems/spiral-matrix/solutions/3506254/rust-o-m-n-solution-with-explanation/
impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];

        while !matrix.is_empty() {
            result.extend(matrix.remove(0));

            for row in 0..matrix.len() {
                if !matrix[row].is_empty() {
                    result.push(matrix[row].pop().unwrap());
                }
            }

            if !matrix.is_empty() {
                result.extend(matrix.pop().unwrap().iter().rev());
            }

            for row in (0..matrix.len()).rev() {
                if !matrix[row].is_empty() {
                    result.push(matrix[row].remove(0));
                }
            }
        }
        result
    }
}
