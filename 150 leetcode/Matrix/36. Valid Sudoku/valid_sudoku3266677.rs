// https://leetcode.com/problems/valid-sudoku/solutions/3266677/rust-solution-using-iterator-trait-adapter/
use std::collections::HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
            let board_len = board.len();

    for i in 0..board_len {
        
        let col_valid = (0..board_len)
                                .map(|idx| board[i][idx])
                                .filter(|c| *c != '.')
                                .is_valid();
        let  row_valid = (0..board_len)
                                .map(|idx| board[idx][i])
                                .filter(|c| *c != '.')
                                .is_valid();
            
        if !row_valid || !col_valid {
            return false
        }
        
        let region_size = 3;

        for row_region in 0..region_size {
            for col_region in 0..region_size {
                let mut items = vec![];
                // Now go through the region
                for i in (row_region * region_size) .. (region_size * (row_region + 1)) {
                    for j in (col_region * region_size) .. (region_size * (col_region + 1)) {
                        
                        items.push(board[i][j]);
                    }
                }
                if !SudokuValid::is_valid(items.into_iter().filter(|c| *c != '.')){
                        return false
                }
            }
        }
    }
    true       
    }
}

trait SudokuValid : Iterator {

    fn is_valid(self) -> bool;
}


impl <T> SudokuValid for T 

    where
        T : Iterator<Item = char>,
{
    fn is_valid(self) -> bool {
        // size hint wont work as expected s if previous one is filter
        // let size = self.size_hint();
        
        let after_filterd = self.map(|c| c).collect::<Vec<_>>();
        let size = after_filterd.len();
        size  == after_filterd.into_iter().collect::<HashSet<_>>().len()
    }
}