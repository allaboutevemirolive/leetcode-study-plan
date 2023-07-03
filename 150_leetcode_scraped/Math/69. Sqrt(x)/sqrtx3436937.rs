// https://leetcode.com/problems/sqrtx/solutions/3436937/rust/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut sqrt:i32 = 0;
        if x == i32::MAX {
            return 46340;
        }
        for i in 0..x + 1{
            
            if i * i == x {
                sqrt = i as i32;
                break
            }
            if ((i * i ) < x) && ((i + 1) * (i + 1)) > x {
                sqrt = i as i32;
                break
            }
        }
      return sqrt;
    }
  
}