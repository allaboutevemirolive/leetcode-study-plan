// https://leetcode.com/problems/snakes-and-ladders/solutions/3092318/rust-elixir-2-bfs-solutions/
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let mut area = board.len() * board.len();
        let mut visited = vec![false; area];
        visited[1] = true;
        let mut v: Vec<usize> = vec![1];
        let mut v2 = Vec::new();
        for turn in 1.. {
            if v.is_empty() {
                return -1;
            }
            for i in v.drain(..) {
                for mut j in i + 1..=i + 6 {
                    let (row, col) = Self::find_pos(j, board.len());
                    if board[row][col] != -1 {
                        j = board[row][col] as usize;
                    }
                    if j == area {
                        return turn;
                    }
                    if !visited[j] {
                        v2.push(j);
                        visited[j] = true;
                    }
                }
            }
            std::mem::swap(&mut v, &mut v2);
        }
        unreachable!()
    }

    fn find_pos(i: usize, width: usize) -> (usize, usize) {
        let div = (i - 1) / width;
        let row = width - 1 - div;
        let mo = (i - 1) % width;
        let col = if div & 1 == 0 { mo } else { width - 1 - mo };
        (row, col)
    }
}