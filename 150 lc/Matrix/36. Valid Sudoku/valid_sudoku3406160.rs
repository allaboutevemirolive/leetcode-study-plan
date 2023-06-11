// https://leetcode.com/problems/valid-sudoku/solutions/3406160/rust-linear-approach-with-constant-time-constraint-checks/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut hal = [0usize; 9];
        let mut val = [0usize; 9];
        let mut bal = [0usize; 9];
        for y in 0..9 {
            for x in 0..9 {
                let c = board[y][x];
                if c != '.' {
                    let bit = 1 << (c as usize - '0' as usize);
                    let z = x / 3 + (y / 3) * 3;
                    if (hal[y] & bit) | (val[x] & bit) | (bal[z] & bit) > 0 { return false; }
                    hal[y] |= bit;
                    val[x] |= bit;
                    bal[z] |= bit;
                }
            }
        }
        return true;
    }
}