// https://leetcode.com/problems/number-of-1-bits/solutions/3066866/two-solutions-rust/
impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut cnt = 0;

        //--------------------------------------------------------
        // Solution 1
        // Modding by 2 returns 0 if first bit is 0 else 1
        // while n > 0 {
        //     cnt+= n%2;
        //     n/= 2;
        // }

        //--------------------------------------------------------
        // Solution 2
        // n-1 is one bit less than n
        // Using the & operator returns a bit as 1 iff both bit positions are 1
        // We can use this to iterate until all of our bits are 0
        while n > 0 {
            cnt+= 1;
            n&= n-1;
        }
        //--------------------------------------------------------
        cnt as i32
    }
}