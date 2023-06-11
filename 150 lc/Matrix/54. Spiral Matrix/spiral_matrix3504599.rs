// https://leetcode.com/problems/spiral-matrix/solutions/3504599/rust-simple-loop-100-runtime/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = if let Some(n) = matrix.first() {
            n.len()
        } else {
            0
        };
        if m == 0 || n == 0 {
            return Vec::new();
        }

        let mut result: Vec<i32> = Vec::with_capacity(m*n);
        result.push(matrix[0][0]);
        let mut cur_dir: u8 = if n > 1 { 1 } else { 2 };
        let mut cur_pos = (0, 0);

        while result.len() < m * n {
            match cur_dir {
                0 => {
                    cur_pos.0 -= 1;
                    if cur_pos.0 - 1 == cur_pos.1 {
                        cur_dir = 1;
                    }
                }
                1 => {
                    cur_pos.1 += 1;
                    if cur_pos.0 == n - (cur_pos.1 + 1) {
                        cur_dir = 2;
                    }
                }
                2 => {
                    cur_pos.0 += 1;
                    if m - cur_pos.0 == n - cur_pos.1 {
                        cur_dir = 3;
                    }
                }
                3 => {
                    cur_pos.1 -= 1;
                    if cur_pos.1 == m - cur_pos.0 - 1 {
                        cur_dir = 0;
                    }
                }
                _ => panic!("Invalid current position"),
            };
            result.push(matrix[cur_pos.0][cur_pos.1]);
        }
        result
    }
}