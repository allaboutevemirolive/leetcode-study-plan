// https://leetcode.com/problems/valid-sudoku/solutions/3143234/the-fastest-rust-solution-i-can-possibly-imagine/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {                
        let mut seen: [bool;768] = [false;768]; // custom 'hash set'

        for i in 0..9 {
            for j in 0..9 {
                let value = board[i][j];
                
                // skip empty cells
                if value == '.' {
                    continue;
                }

                let b = (i / 3) + (j / 3) * 3; // current 3x3 box index
                let n = value as usize & 0xF; // retrieve number from char

                // hashset record index works as follows:
                // vvvv pppp rr
                // - v: value
                // - p: position (row, column or box)
                // - r: record type (row = 0, column = 1, box = 2)

                // so we can calculate it using:
                // idx = n | (i << 4) | (r << 8)

                let row_idx = n | (i << 4);
                if seen[row_idx] {
                    return false; // value already exists in this row -> sudoku is invalid
                }
                seen[row_idx] = true; // update record
                
                // repeat for column and box
                let col_idx = n | (j << 4) | 0x100;
                if seen[col_idx] {
                    return false;
                }
                seen[col_idx] = true;
                
                let box_idx = n | (b << 4) | 0x200;
                if seen[box_idx] {
                    return false;
                }
                seen[box_idx] = true;
            }
        }
        true // no conflicts found
    }
}
