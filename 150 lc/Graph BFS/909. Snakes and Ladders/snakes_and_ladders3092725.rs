// https://leetcode.com/problems/snakes-and-ladders/solutions/3092725/rust-somewhat-slow-easy-bfs-solution/
use std::collections::HashSet;

// simple board structure that exposes a friendly api
// maps positions to  "coordinates" to easily get values
struct Board {
    board: Vec<Vec<i32>>,
    size: usize,
}
impl Board {
    fn new(board: Vec<Vec<i32>>) -> Board {
        let size = board.len() * board.len();
        Board { board, size }
    }
    fn value_at(&self, pos: usize) -> i32 {
        let (row, col) = self.position_to_coords(pos);
        self.board[row][col]
    }
    fn is_at_end(&self, pos: usize) -> bool {
        pos == self.size
    }
    fn position_to_coords(&self, pos: usize) -> (usize, usize) {
        let pos = pos - 1;
        let row = self.board.len() - pos / self.board.len() - 1;
        let col = pos % self.board.len();
        if (self.board.len() - row) % 2 == 1 {
            (row, col)
        } else {
            (row, self.board.len() - col - 1)
        }
    }
    // get all next positions, if the next position is 
    // snake or ladder, use that position instead
    // I couldn't make it return an Iterator :'(
    fn get_next_positions(&self, pos: &usize) -> Vec<usize> {
        (pos + 1..=self.size.min(pos + 6))
            .map(|p| {
                let val = self.value_at(p);
                if val == -1 {
                    p
                } else {
                    val as usize
                }
            })
            .collect()
    }
}

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        Solution::min_stesp_to_end(&Board::new(board))
    }
    fn min_stesp_to_end(board: &Board) -> i32 {
        let mut pos_queue = HashSet::new();
        let mut visited = HashSet::new();
        pos_queue.insert(1usize);

        let mut step_counter = 0;
        while !pos_queue.is_empty() {
            for p in pos_queue.iter() {
                visited.insert(*p);
                if board.is_at_end(*p) {
                    return step_counter;
                }
            }
            pos_queue = pos_queue
                .iter()
                .map(|p| board.get_next_positions(p))
                .flatten()
                .filter(|p| !visited.contains(p))
                .collect();
            step_counter += 1;
        }

        -1
    }
}