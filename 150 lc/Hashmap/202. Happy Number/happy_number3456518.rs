// https://leetcode.com/problems/happy-number/solutions/3456518/rust/
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut num = n;
        let mut sum = 0;
        let mut set: HashSet<i32> = HashSet::new();
        loop {
            if set.contains(&num) {
                return false;
            }

            set.insert(num);
            while num > 0 {
                sum += (num % 10) * (num % 10);
                num /= 10;
            }

            if sum == 1 {
                return true;
            }

            num = sum;
            sum = 0;
        }
    }
}