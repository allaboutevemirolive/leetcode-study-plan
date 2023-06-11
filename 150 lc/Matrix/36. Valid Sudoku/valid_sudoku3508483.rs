// https://leetcode.com/problems/valid-sudoku/solutions/3508483/simple-rust-solution/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rowHashes = Solution::empty_hash_sets();
        let mut colHashes = Solution::empty_hash_sets();
        let mut boxHashes = Solution::empty_hash_sets();

        for (rowIdx, row) in board.iter().enumerate() {
            for (colIdx, &ch) in row.iter().enumerate() {
                let boxIdx = (rowIdx / 3) * 3 + colIdx / 3;

                if ch == '.' {
                    continue;
                }

                if  !rowHashes[rowIdx].insert(ch) ||
                    !colHashes[colIdx].insert(ch) ||
                    !boxHashes[boxIdx].insert(ch) {
                    return false;
                }
            }
        }

        true
    }

    fn empty_hash_sets() -> [std::collections::HashSet<char>; 9] {
        Default::default()
    }
}