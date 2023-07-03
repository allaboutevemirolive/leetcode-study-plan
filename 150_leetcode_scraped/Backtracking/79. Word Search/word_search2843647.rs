// https://leetcode.com/problems/word-search/solutions/2843647/rust-non-recursive-recursive-backtracking/
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) 
        -> bool 
    {
        let n = board.len();
        let m = board[0].len();
        let w = word.chars().collect::<Vec<char>>();
        let mut visited = vec![vec![false; m]; n];
        for i in 1..=n {
            for j in 1..=m {
                if board[i - 1][j - 1] == w[0] {
                    if Self::rummage(&board, &w, &mut visited, i, j, 0) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn rummage(board    : &[Vec<char>],
               word     : &[char], 
               visited  : &mut [Vec<bool>], 
               i        : usize, 
               j        : usize, 
               k        : usize) 
        -> bool 
    {
        if k == word.len() {
            true
        }
        else if i < 1 || i > board.len() || 
                j < 1 || j > board[0].len() || 
                visited[i - 1][j - 1] || board[i - 1][j - 1] != word[k] 
        {
            false
        }
        else {
            visited[i - 1][j - 1] = true;
            let res = Self::rummage(board, word, visited, i + 1, j, k + 1) ||
                      Self::rummage(board, word, visited, i - 1, j, k + 1) ||
                      Self::rummage(board, word, visited, i, j + 1, k + 1) ||
                      Self::rummage(board, word, visited, i, j - 1, k + 1);
            visited[i - 1][j - 1] = false;
            res
        }
    }
}