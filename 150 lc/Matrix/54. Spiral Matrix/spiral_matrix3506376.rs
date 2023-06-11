// https://leetcode.com/problems/spiral-matrix/solutions/3506376/rust-py-c-go-track-direction-and-region-from-which-to-which-to-get-elements/
impl Solution {
  pub fn spiral_order(M: Vec<Vec<i32>>) -> Vec<i32> {
    let (mut y1, mut x1, mut y2, mut x2) = (0, 0, M.len(), M[0].len());
    let mut dir = 0;
    let total = x2 * y2;
    let mut res = Vec::with_capacity(total);

    while res.len() < total {
      if dir == 0 {
        for x in x1 .. x2 {
          res.push(M[y1][x]);
        }
        y1 += 1;
        dir = 1;
      } else if dir == 1 {
        for y in y1 .. y2 {
          res.push(M[y][x2 - 1]);
        }
        x2 -= 1;
        dir = 2;
      } else if dir == 2 {
        for x in (x1 .. x2).rev() {
          res.push(M[y2 - 1][x]);
        }
        y2 -= 1;
        dir = 3;
      } else {
        for y in (y1 .. y2).rev() {
          res.push(M[y][x1]);
        }
        x1 += 1;
        dir = 0
      }
    }

    return res 
  }
}