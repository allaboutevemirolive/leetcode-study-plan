// https://leetcode.com/problems/valid-sudoku/solutions/3542632/rust-solution-hashmap/
use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut hash: HashMap<char, bool> = HashMap::new();

        for rows in &board {
            for c in rows {
                if *c == '.' {
                    continue;
                }
                match hash.insert(*c, true) {
                    Some(_) => return false,
                    None => continue,
                }
            }

            hash = HashMap::new();
        }

        for col in 0..9 {
            for row in 0..9 {
                let c = &board[row][col];
                if *c == '.' {
                    continue;
                }
                match hash.insert(*c, true) {
                    Some(_) => return false,
                    None => drop(c),
                }
            }

            hash = HashMap::new();
        }

        for p in 0..9 {
            let r_offset = p / 3;
            let c_offset = p % 3;
            for x in 0..9 {
                let row = x / 3;
                let col = x % 3;

                let c = &board[row + r_offset * 3][col + c_offset * 3];

                if *c == '.' {
                    continue;
                }

                match hash.insert(*c, true) {
                    Some(_) => return false,
                    None => drop(c),
                }
            }

            hash = HashMap::new();
        }

        true
    }
}