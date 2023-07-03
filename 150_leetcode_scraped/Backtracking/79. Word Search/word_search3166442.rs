// https://leetcode.com/problems/word-search/solutions/3166442/rust-backtracking-dfs-solution/
impl Solution {
    pub fn solve(board : &mut Vec<Vec<char>>, word : &mut String,i : i32,j : i32,k : i32,vis : &mut Vec<Vec<bool>>) -> bool{

        if k  as usize == word.len() {
            return true;
        }

        if i as usize >= board.len() || j as usize >= board[0].len() || board[i as usize][j as usize] != word.chars().nth(k as usize).unwrap() || vis[i as usize][j as usize] {
            return false;
        }
        // println!("{}",board[i][j]);
        let mut ans = false;

        if board[i as usize][j as usize] == word.chars().nth(k as usize).unwrap(){
            let dx : [i32;4] = [-1,1,0,0];
            let dy : [i32;4] = [0,0,-1,1];
            vis[i as usize ][j as usize] = true;
            for z in 0..4 {
                let xx : i32 = (dx[z] + i as i32 ) as i32;
                let yy : i32 = (dy[z] + j as i32) as i32;

                ans |= Self::solve(board,word,xx,yy,k + 1,vis);
            }
            vis[i as usize ][j as usize] = false;
        }
        return ans;
    }

    pub fn exist(mut board: Vec<Vec<char>>,mut word: String) -> bool {
        let mut vis : Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];

        for x in 0..board.len() {
            for y in 0..board[0].len() {
                let mut ans = Self::solve(&mut board,&mut word,x as i32,y as i32,0 as i32,&mut vis);
                if ans {
                    return true;
                }
            }
        }
        false
    }
}