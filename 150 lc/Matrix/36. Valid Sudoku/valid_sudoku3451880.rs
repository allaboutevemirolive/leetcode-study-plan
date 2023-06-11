// https://leetcode.com/problems/valid-sudoku/solutions/3451880/rust-3-pass-solution-columns-rows-3x3-grids/
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Check rows
        for y in 0..board.len() {
            let mut row = HashSet::new();
            for x in 0..board[y].len() {
                let v = board[y][x];
                if v == '.' {
                    continue;
                }
                if row.contains(&v) {
                    return false;
                }
                row.insert(v);
            }
        }
        // Check columns
        for y in 0..board.len() {
            let mut col = HashSet::new();
            for x in 0..board[y].len() {
                let v = board[x][y];
                if v == '.' {
                    continue;
                }
                if col.contains(&v) {
                    return false;
                }
                col.insert(v);
            }
        }
        // Check 3x3 grids
        let (mut start_x, mut end_x, mut start_y, mut end_y) = (0, 3, 0, 3);
        loop {
            let mut grid = HashSet::new();
            for y in start_y..end_y {
                for x in start_x..end_x {
                    let v = board[y][x];
                    if v == '.' {
                        continue;
                    }
                    if grid.contains(&v) {
                        return false;
                    }
                    grid.insert(v);
                }
            }
            if end_x == board.len() && end_y == board.len() {
                break;
            }
            if end_x == board.len() {
                start_y += 3;
                end_y += 3;
                start_x = 0;
                end_x = 3;
            } else {
                start_x += 3;
                end_x = std::cmp::min(end_x + 3, board.len());
            }
        }
        true
    }
}
