// https://leetcode.com/problems/happy-number/solutions/2651347/rust-0ms-cycle-detection-closure/
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut x = n;
        let mut y = n;
        let step = |mut x: i32| {
            let mut s = 0;
            while x > 0 {
                s += (x % 10) * (x % 10);
                x /= 10;
            }
            s
        };
        loop {
            x = step(x);
            y = step(step(y));
            if x == y {
                break
            }
        }
        x == 1
    }
}