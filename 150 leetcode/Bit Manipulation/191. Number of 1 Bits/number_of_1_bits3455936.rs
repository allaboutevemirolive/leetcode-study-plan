// https://leetcode.com/problems/number-of-1-bits/solutions/3455936/rust-solution-with-right-shifting/
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut ans = 0;
        let mut tmp = n;
        while tmp>0 {
            if tmp & 1 == 1 {
                ans+=1;
            }
            tmp >>= 1;
        }
        return ans;
    }
}