// https://leetcode.com/problems/spiral-matrix/solutions/3503962/rust-easy-solution/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let mut res = vec![];
        let (mut l, mut r, mut u, mut d) = (0, n - 1, 0, m - 1);
        let mut dir = 0;

        while l <= r && u <= d {
            match dir % 4 {
                0 => {
                    for i in l..r + 1 {
                        res.push(matrix[u as usize][i as usize]);
                    }
                    u += 1;
                }
                1 => {
                    for i in u..d + 1 {
                        res.push(matrix[i as usize][r as usize]);
                    }
                    r -= 1;
                }
                2 => {
                    for i in (l..r + 1).rev().step_by(1) {
                        res.push(matrix[d as usize][i as usize]);
                    }
                    d -= 1;
                }
                _ => {
                    for i in (u..d + 1).rev().step_by(1) {
                        res.push(matrix[i as usize][l as usize]);
                    }
                    l += 1;
                }
            }

            dir += 1;
        }

        res
    }
}