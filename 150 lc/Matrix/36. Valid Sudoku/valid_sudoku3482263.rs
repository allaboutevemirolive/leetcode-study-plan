// https://leetcode.com/problems/valid-sudoku/solutions/3482263/simple-rust-solution/
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        Self::rows_valid(&board) && Self::columns_valid(&board) && Self::boxes_valid(&board)
    }

    fn rows_valid(board: &Vec<Vec<char>>) -> bool {
        for row in board {
            let row_numbers = row.iter().filter(|x: &&char| **x != '.');
            if row_numbers
                .clone()
                .filter(|num: &&char| num.is_ascii_digit() && **num != '0')
                .count()
                != HashSet::from(row_numbers.collect::<HashSet<&char>>()).len()
            {
                return false;
            }
        }
        true
    }

    fn columns_valid(board: &Vec<Vec<char>>) -> bool {
        let rotated_board = Self::rotate_board(board);
        Self::rows_valid(&rotated_board)
    }

    fn boxes_valid(board: &Vec<Vec<char>>) -> bool {
        let boxed_board = Self::board_to_boxes(board);
        Self::rows_valid(&boxed_board)
    }

    fn rotate_board(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_board: Vec<Vec<char>> = Vec::new();
        for i in 0..9 {
            new_board.push(Vec::new());
            for j in 0..9 {
                new_board[i].push(board[j][i]);
            }
        }
        new_board
    }

    fn board_to_boxes(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_board: Vec<Vec<char>> = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                new_board.push(Vec::new());
                for k in 0..3 { 
                    for l in 0..3 {
                        new_board[i*3 + j].push(board[k + i*3][l + j*3]); 
                    }
                }
            }
        }
        new_board
    }
}