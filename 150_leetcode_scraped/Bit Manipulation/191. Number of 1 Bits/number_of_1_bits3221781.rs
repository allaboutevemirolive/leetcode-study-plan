// https://leetcode.com/problems/number-of-1-bits/solutions/3221781/rust-recursive-kinda-dumb-tho/
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        if n==0 {
            0
        }else if n&1==1{
            Solution::hammingWeight(n>>1)+1
        }else{
            Solution::hammingWeight(n>>1)
        }
        
    }
}