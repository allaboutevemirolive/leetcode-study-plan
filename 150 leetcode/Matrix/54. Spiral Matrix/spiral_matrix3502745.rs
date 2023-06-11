// https://leetcode.com/problems/spiral-matrix/solutions/3502745/rust/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut left = 0;
        let mut right = matrix[0].len() - 1;
        let mut top = 0;
        let mut bottom = matrix.len() - 1;

        let mut ret = Vec::with_capacity(matrix.len() * matrix[0].len());

        loop {
            for i in (left..=right) {
                ret.push(matrix[top][i]);
            }
            if top == bottom {
                break;
            }
            top += 1;

            for i in top..=bottom {
                ret.push(matrix[i][right]);
            }
            if left == right {
                break;
            }
            right -= 1;

            for i in (left..=right).rev() {
                ret.push(matrix[bottom][i]);
            }
            if top == bottom {
                break;
            }
            bottom -= 1;

            for i in (top..=bottom).rev() {
                ret.push(matrix[i][left]);
            }
            if left == right {
                break;
            }
            left += 1;
        }

        ret
    }
}