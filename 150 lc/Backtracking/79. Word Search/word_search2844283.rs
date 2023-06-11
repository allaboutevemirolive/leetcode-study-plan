// https://leetcode.com/problems/word-search/solutions/2844283/rust-intuitive-backtracking-solution/
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn helper(r: i32, c: i32, soln: &Vec<char>, mut cur_soln: &mut Vec<char>, board: &mut Vec<Vec<char>>) -> bool {
            if soln.len() == cur_soln.len() {
                return true;
            }
            
            // oob
            if r < 0 || r >= board.len() as i32 || c < 0 || c >= board[0].len() as i32 {
                return false;
            }
                        
            // len + visited check
            if board[r as usize][c as usize] == '.' || cur_soln.len() >= soln.len() {
                return false;
            }
            
            let tmp = board[r as usize][c as usize];
            
            // check if correct next value
            if soln[cur_soln.len()] != tmp {
                return false;
            }
            
            cur_soln.push(tmp);
            board[r as usize][c as usize] = '.';
            
            let is_valid_path = [
                helper(r+1, c, soln, cur_soln, board),
                helper(r-1, c, soln, cur_soln, board),
                helper(r, c+1, soln, cur_soln, board),
                helper(r, c-1, soln, cur_soln, board),
            ].contains(&true);
                        
            cur_soln.pop();
            board[r as usize][c as usize] = tmp;
            
            is_valid_path
        }
        
        let word = word.chars().collect::<Vec<char>>();
        let mut soln = vec![];
        
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if helper(r as i32, c as i32, &word, &mut soln, &mut board) == true {
                    return true;
                }
            }
        }
        
        false
    }
}