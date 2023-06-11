// https://leetcode.com/problems/happy-number/solutions/3589406/rust-solution-with-cycle-test-slow-and-fast-pointer/
impl Solution {
    fn get_next(mut n: i32) -> i32 {
        let mut sum = 0;
        while n != 0 {
            let rem = n % 10;
            sum += rem*rem;
            n = n / 10;
        }
        sum
    }
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::get_next(n);
        while slow != 1 && slow != fast {
            slow = Self::get_next(slow);
            fast = Self::get_next(Self::get_next(fast));
        }
        slow == 1
    }
}