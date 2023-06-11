// https://leetcode.com/problems/spiral-matrix/solutions/3506971/multithread-compatible-rust-solution/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; matrix.len() * matrix[0].len()];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                let idx = Self::get_spiral_idx(row, col, matrix.len(), matrix[0].len());
                result[idx] = matrix[row][col];
            }
        }

        result
    }

    fn get_spiral_idx(row: usize, col: usize, rows: usize, cols: usize) -> usize {
        let layer = Self::get_layer(row, col, rows, cols);

        let max_layer_row = if rows > layer { rows - layer - 1 } else { 0 };
        let max_layer_col = if cols > layer { cols - layer - 1 } else { 0 };

        let layer_width = if layer <= max_layer_col {
            max_layer_col - layer + 1
        } else {
            0
        };
        let layer_height = if layer <= max_layer_row {
            max_layer_row - layer + 1
        } else {
            0
        };

        let total_prev_layer_elements = rows * cols - (rows - 2 * layer) * (cols - 2 * layer);

        if row == layer {
            // Top side
            total_prev_layer_elements + (col - layer)
        } else if col == max_layer_col {
            // Right side
            total_prev_layer_elements + layer_width + row - layer - 1
        } else if row == max_layer_row {
            // Bottom side
            total_prev_layer_elements + layer_width + layer_height + max_layer_col - col - 2
        } else {
            // Left side
            total_prev_layer_elements + layer_width + layer_height + layer_width + max_layer_row
                - row
                - 3
        }
    }

    fn get_layer(row: usize, col: usize, rows: usize, cols: usize) -> usize {
        row.min(col).min(rows - row - 1).min(cols - col - 1)
    }
}