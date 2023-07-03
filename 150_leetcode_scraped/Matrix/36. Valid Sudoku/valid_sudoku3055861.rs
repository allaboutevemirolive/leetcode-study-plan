// https://leetcode.com/problems/valid-sudoku/solutions/3055861/rust-bit-masking-100/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        const N : usize = 9;
        let (mut rows, mut cols, mut squares) : ([u16;N], [u16;N], [u16;N]) = ([0;N], [0;N], [0;N]);
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if let Some(val) = board[r][c].to_digit(10) {
                    let g = (r/3) * 3 + (c / 3);
                    let idx = (val - 1) as usize;
                    if (rows[r] & (1 << idx)) > 0 || (cols[c] & ( 1 << idx)) > 0 || (squares[g] & ( 1 << idx)) > 0 {
                        return false;
                    }
                    rows[r] |= ( 1 << idx);
                    cols[c] |= ( 1 << idx);
                    squares[g] |= ( 1 << idx);
                }
            }
        }
        true
    }
}