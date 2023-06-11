// https://leetcode.com/problems/happy-number/solutions/3489077/rust-solution/
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        #[inline]
        fn get_next(mut n: i32) -> i32 {
            let mut ans = 0;
            while n > 0 {
                ans += (n % 10).pow(2);
                n /= 10;
            }
            ans
        }
        use std::collections::HashSet;
        let mut set = HashSet::new();
        while n != 1 {
            set.insert(n);
            let next = get_next(n);
            if set.contains(&next) {
                return false;
            }
            n = next;
        }
        true 
    }
}