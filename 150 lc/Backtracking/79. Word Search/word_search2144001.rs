// https://leetcode.com/problems/word-search/solutions/2144001/rust-backtracking-nice-and-concise-with-functional-style/
impl Solution {
    fn dfs(board: &[Vec<char>], word: &[char], visited: &mut [Vec<bool>], m: usize, n: usize, len: usize, i: usize, j: usize, depth: usize) -> bool {
        depth == len || (i < m && j < n && !visited[i][j] && word[depth] == board[i][j] && {
            visited[i][j] = true;
            let rez = [0, 1, 0, !0, 0]
                .windows(2)
                .any(|w| Self::dfs(board, word, visited, m, n, len, i.wrapping_add(w[0]), j.wrapping_add(w[1]), depth + 1));
            visited[i][j] = false;
            rez
        })
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let word = word.chars().collect::<Vec<_>>();
        let len = word.len();
        let mut visited = vec![vec![false; n]; m];

        (0..m).any(|i| (0..n).any(|j| Self::dfs(&board, &word, &mut visited, m, n, len, i, j, 0)))
    }
}