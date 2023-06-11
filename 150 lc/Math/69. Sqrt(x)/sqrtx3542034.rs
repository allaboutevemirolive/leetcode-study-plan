// https://leetcode.com/problems/sqrtx/solutions/3542034/really-simple-rust-binary-search/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {return x}
      let (mut lower, mut upper) = (1, 46341);
      while upper > 1 + lower {
          let new = (lower + upper) >> 1;
          if new*new > x {upper = new}
          else {lower = new}
      }
      return lower
    }
}