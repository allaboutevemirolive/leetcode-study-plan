// https://leetcode.com/problems/game-of-life/solutions/3589757/simple-and-easy-solution-0-mn-rust-tm-noob/
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut r: i32 = 0;
        let mut changes_position: Vec<Vec<i32>> = vec![];
        /// [[r, c, value], ...]
        while r < board.len() as i32{
            let mut c: i32 = 0;
            while c < board[0].len() as i32{
                // check neighbor
                let mut count_live_neighbor: i32 = 0;
                // eight side
                // horizontal
                if c+1 < board[0].len() as i32 && board[r as usize][(c+1) as usize] == 1{
                    count_live_neighbor += 1;
                }
                if c-1 >= 0 && board[r as usize][(c-1) as usize] == 1{
                    count_live_neighbor += 1;
                }
                // vertical
                if r+1 < board.len() as i32 && board[(r+1) as usize][c as usize] == 1{
                    count_live_neighbor += 1;
                }
                if r-1 >= 0 && board[(r-1) as usize][c as usize] == 1{
                    count_live_neighbor += 1;
                }
                // diagonal four
                // primary
                if r + 1 < board.len() as i32 && c + 1 < board[0].len() as i32 && board[(r+1) as usize][(c+1) as usize] == 1{
                    count_live_neighbor += 1;
                }
                if r - 1 >= 0 && c - 1 >= 0 && board[(r-1) as usize][(c-1) as usize] == 1{
                    count_live_neighbor += 1;
                }
                // secondary
                if r - 1 >= 0 && c + 1 < board[0].len() as i32 && board[(r-1) as usize][(c+1) as usize] == 1{
                    count_live_neighbor += 1;
                }
                if r + 1 < board.len() as i32 && c - 1 >= 0 && board[(r+1) as usize][(c-1) as usize] == 1{
                    count_live_neighbor += 1;
                }
                // println!("alive : {}", count_live_neighbor);

                // live cell died
                if (count_live_neighbor > 3 || count_live_neighbor < 2) && board[r as usize][c as usize] == 1{
                    changes_position.push(vec![r,c,0, count_live_neighbor]);
                }
                // dead cell live
                if count_live_neighbor == 3 && board[r as usize][c as usize] == 0{
                    changes_position.push(vec![r,c,1, count_live_neighbor]);
                }

                c += 1;
            }
            r += 1;
        }    
        // println!("{:?}", changes_position);     
        // change the board
        for i in 0..changes_position.len(){
            board[changes_position[i][0] as usize][changes_position[i][1] as usize] = changes_position[i][2];
        }
    }
}