// https://leetcode.com/problems/valid-sudoku/solutions/3402086/rust-constraints-check-for-every-filled-cell-using-additional-arrays/
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut hal = vec![Vec::<char>::new(); 9];
        let mut val = vec![Vec::<char>::new(); 9];
        let mut bal = vec![Vec::<char>::new(); 9];
        for y in 0..9 {
            for x in 0..9 {
                let c = board[y][x];
                if c != '.' {
                    let z = x / 3 + (y / 3) * 3;
                    if hal[y].contains(&c) || val[x].contains(&c) || bal[z].contains(&c) { return false; }
                    hal[y].push(c);
                    val[x].push(c);
                    bal[z].push(c);
                }
            }
        }
        return true;
    }
}