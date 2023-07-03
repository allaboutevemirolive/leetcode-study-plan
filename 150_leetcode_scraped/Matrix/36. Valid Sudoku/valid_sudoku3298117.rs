// https://leetcode.com/problems/valid-sudoku/solutions/3298117/extremely-readable-rust-o-1/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut digit_present_in_row: [[bool; 9]; 9] = Default::default();
        let mut digit_present_in_col: [[bool; 9]; 9] = Default::default();
        let mut digit_present_in_quarter: [[[bool; 9]; 3]; 3] = Default::default();
        for x in 0..9 {
            for y in 0..9 {
                if board[x][y] == '.' {
                    continue;
                }
                let digit: usize = (board[x][y].to_digit(10).unwrap() - 1) as usize;

                if digit_present_in_row[x][digit] {
                    return false;
                } else {
                    digit_present_in_row[x][digit] = true;
                }

                if digit_present_in_col[y][digit] {
                    return false;
                } else {
                    digit_present_in_col[y][digit] = true;
                }

                if digit_present_in_quarter[x / 3][y / 3][digit] {
                    return false;
                } else {
                    digit_present_in_quarter[x / 3][y / 3][digit] = true;
                }
            }
        }
        true
    }
}