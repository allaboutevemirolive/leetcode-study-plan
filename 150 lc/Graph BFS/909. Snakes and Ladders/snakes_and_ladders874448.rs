// https://leetcode.com/problems/snakes-and-ladders/solutions/874448/rust-translated-4ms-100/
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        fn get_board_value(board: &[Vec<i32>], num: i32) -> i32 {
            let n = board.len() as i32;
            let r = (num - 1) / n;
            let x = n - 1 - r;
            let y = if r % 2 == 0 {
                num - 1 - r * n
            } else {
                n + r * n - num
            };
            board[x as usize][y as usize]
        }

        let n = board.len();
        let mut queue = VecDeque::<i32>::new();
        queue.push_back(1);
        let mut visited = vec![false; n * n + 1];
        let mut ans = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let num = queue.pop_front().unwrap();
                if visited[num as usize] {
                    continue;
                }
                visited[num as usize] = true;
                if num as usize == n * n {
                    return ans;
                }
                (1..=6).for_each(|i| {
                    let mut next = num + i;
                    if next <= (n * n) as i32 {
                        let value = get_board_value(&board, next);
                        if value > 0 {
                            next = value;
                        }
                        if !visited[next as usize] {
                            queue.push_back(next);
                        }
                    }
                });
            }
            ans += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snakes_and_ladders() {
        assert_eq!(
            Solution::snakes_and_ladders(vec![
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 35, -1, -1, 13, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 15, -1, -1, -1, -1]
            ]),
            4
        );
    }

    #[test]
    fn test_snakes_and_ladders_02() {
        assert_eq!(
            Solution::snakes_and_ladders(vec![
                vec![-1, 1, 2, -1],
                vec![2, 13, 15, -1],
                vec![-1, 10, -1, -1],
                vec![-1, 6, 2, 8],
            ]),
            2
        );
    }
}