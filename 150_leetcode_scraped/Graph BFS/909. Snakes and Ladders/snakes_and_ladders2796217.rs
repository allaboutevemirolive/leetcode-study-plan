// https://leetcode.com/problems/snakes-and-ladders/solutions/2796217/rust-solution-using-dijkstra/
impl Solution {
    pub fn snakes_and_ladders(mut board: Vec<Vec<i32>>) -> i32 {
        board.reverse();
        let n = board.len();
        let inf = 1_000_000_000;
        let mut memo = vec![inf;n*n];
        memo[0] = 0;
        let mut stack = vec![(0,0)];
        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some((ci, v)) = stack.pop() {
                let nv = v+1;
                let limit = std::cmp::min(ci+6, n*n-1);
                for ni in ci+1..=limit {
                    let r = ni / n;
                    let c = if r % 2 == 0 {
                        ni % n
                    } else {
                        n - 1 - ni % n
                    };

                    if board[r][c] != -1 {
                        let ni = board[r][c] as usize - 1;
                        if nv < memo[ni] {
                            memo[ni] = nv;
                            new_stack.push((ni, nv));
                        }
                    } else if nv < memo[ni] {
                        memo[ni] = nv;
                        new_stack.push((ni, nv));
                    }
                }
            }
            stack = new_stack;
        }

        if memo[n*n-1] == inf {
            -1
        } else {
            memo[n*n-1]
        }
    }
}