// https://leetcode.com/problems/happy-number/solutions/3380943/rust-0ms/
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        loop {
            match n {
                1 => return true,
                4 => return false,
                _ => {
                    let mut x = 0;
                    while n > 0 {
                        x += (n % 10) * (n % 10);
                        n = n / 10;
                    }
                    n = x
                }
            }
        }
    }
}