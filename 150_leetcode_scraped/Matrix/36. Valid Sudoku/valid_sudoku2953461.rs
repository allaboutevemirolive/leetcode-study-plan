// https://leetcode.com/problems/valid-sudoku/solutions/2953461/rust-beats-100-99-8/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in &board {
            if !check_row(row.to_vec()) {
                return false;
            }
        };
        if !check_collum(&board) {
            return false;
        };

        if !check_squares(&board) {
            return false;
        };
        return true;
    }
}

fn check_squares(board: &Vec<Vec<char>>) -> bool {
    let mut square = ['.'; 9];
    let mut index = 0;
    for i in 0..3 {
        for o in 0..3 {
            index = 0;
            for j in (i * 3)..(i * 3 + 3) {
                for k in (o * 3)..(o * 3 + 3) {
                    square[index] = board[j][k];
                    index += 1;
                }
            }
            if !check_row(square.to_vec()) {
                return false;
            }
        }
    }
    return true;
}

fn check_collum(board: &Vec<Vec<char>>) -> bool {
    let mut transposed = [['.'; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            transposed[i][j] = board[j][i];
        }
    }
    for row in transposed {
        if !check_row(row.to_vec()) {
            return false;
        }
    }
    return true;
}
fn check_row(row: Vec<char>) -> bool {
    for char in &row {
        if row.iter().filter(|&c| *c == *char).count() > 1 {
            if *char == '.' {
                continue;
            }
            return false;
        }
    }
    return true;
}
