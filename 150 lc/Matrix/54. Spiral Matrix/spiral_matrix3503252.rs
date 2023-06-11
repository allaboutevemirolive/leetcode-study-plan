// https://leetcode.com/problems/spiral-matrix/solutions/3503252/rust-match/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut out = vec![];
        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;

        let mut xy = (0,0);
        let mut way = (0, 1);
        let (mut top, mut bottom, mut left, mut right) = (0, m-1, -1, n-1);

        while (out.len() as i32) < m * n {
            match xy {
                xy if xy == (top, left) => {
                    way = (0,1); // →
                    bottom -= 1;
                },
                xy if xy == (top, right) => {
                    way = (1,0); // ↓
                    left += 1;
                },
                xy if xy == (bottom, right) => {
                    way = (0,-1); // ←
                    top += 1;
                },
                xy if xy == (bottom, left) => {
                    way = (-1,0); // ↑
                    right -= 1;
                },
                _ => {},
            }

            out.push(matrix[xy.0 as usize][xy.1 as usize]);
            xy = (xy.0 + way.0, xy.1 + way.1);
        }
        
        return out;
    }
}