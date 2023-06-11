// https://leetcode.com/problems/valid-sudoku/solutions/3439471/rust-solution-0ms-runtime/
use std::collections::HashSet;
use std::ops::Range;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {

        if Solution::has_line_repeats(&board) {
            return false;
        }

        let ranges = vec![0..3, 3..6, 6..9];

        for row_range in 0..ranges.len() {
            for column_range in 0..ranges.len() {
                if Solution::has_box_repeats (&board, ranges[row_range].clone(), ranges[column_range].clone()) {
                    return false;
                }
            }
        }

        true
    }

    fn has_line_repeats(board: &Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::<char>::new();9];
        let mut columns = vec![HashSet::<char>::new();9];

        for fixed_element in 0..9 {
            for moving_element in 0..9 {
                let current_row_element = board[fixed_element][moving_element];
                let current_column_element = board[moving_element][fixed_element];

                if let Err(_) = Solution::checked_insert(current_row_element, &mut rows[fixed_element]) {
                    return true;
                }
                if let Err(_) = Solution::checked_insert(current_column_element, &mut columns[fixed_element]) {
                    return true;
                }
            }
        }

        return false;
    }

    fn has_box_repeats(board: &Vec<Vec<char>>, row_range: Range<usize>, column_range: Range<usize>) -> bool {
        let mut box_set = HashSet::<char>::new();
        for row_index in row_range {
            for column_index in column_range.clone() {
                let curr = board[row_index][column_index];
                if let Err(_) = Solution::checked_insert(curr, &mut box_set) {
                    return true;
                }
            }
        }

        false
    }

    fn checked_insert(curr: char, set: &mut HashSet<char>) -> Result<(), ()> {
        if curr != '.' {
            if set.contains(&curr) {
                return Err(());
            }

            set.insert(curr);
        }

        Ok(())
    }
}
