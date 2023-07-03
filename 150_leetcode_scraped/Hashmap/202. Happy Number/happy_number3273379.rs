// https://leetcode.com/problems/happy-number/solutions/3273379/rust-solution-0ms/
pub fn calcu(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let d = n % 10;
            res += d * d;
            n /= 10;
        }
        res
    }
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut m = n;
        let mut v:Vec<i64> = Vec::new();
        let mut sum = 0;
        if (n == 1) {return true;}
        loop {
            
               sum = calcu(m);
               if v.contains(&(sum as i64)) {
                   return false;
               }
               v.push(sum as i64);
               if (sum == 1){
                return true;
                }
                m = sum;
        }
    }
}