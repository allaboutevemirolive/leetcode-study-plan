// https://leetcode.com/problems/word-search/solutions/3183097/rust-solution-without-out-of-board-checking/
impl Solution {
    pub fn exist_sub(x: usize, y: usize, m:&mut [[u8;8]; 8], w: &[u8]) -> bool {
        if w.len() == 0 {
            return true;
        }
        if m[y][x] != w[0] {
            return false;
        }
        if w.len() < 2 {
            return true;
        }        
        m[y][x] = 0;
        if Solution::exist_sub(x + 1, y + 0, m, &w[1..]) {return true};
        if Solution::exist_sub(x - 1, y + 0, m, &w[1..]) {return true};
        if Solution::exist_sub(x + 0, y + 1, m, &w[1..]) {return true};
        if Solution::exist_sub(x + 0, y - 1, m, &w[1..]) {return true};
        m[y][x] = w[0];
        return false;
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let sy = board.len();
        let sx = board[0].len();
        if word.len() > (sx*sy) {
            return false;
        }
        
        let mut m = [[0x00; 8]; 8]; // fill with less bigger border with shift x+1 y+1 for avoid range checking out of boarder
        for (y, v) in board.iter().enumerate() {
            for (x, c) in v.iter().enumerate() {
                m[1 + y][1 + x] = *c as u8;
            }
        }
        let w = word.as_bytes();
        for y in 1..(sy+1) {
            for x in 1..(sx+1) {
                if Solution::exist_sub(x,y, &mut m, w) {
                    return true
                }
            }    
        }
        return false;
    }
}