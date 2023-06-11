// https://leetcode.com/problems/happy-number/solutions/2809626/leetcode-the-hard-way-rust-floyd-s-cycle-finding-algorithm/
impl Solution {
    fn nxt(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let d = n % 10;
            res += d * d;
            n /= 10;
        }
        res
    }
    
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Solution::nxt(n);
        while fast != 1 && slow != fast {
            slow = Solution::nxt(slow);
            fast = Solution::nxt(Solution::nxt(fast));
        }
        return fast == 1;
    }
}