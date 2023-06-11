// https://leetcode.com/problems/house-robber/solutions/3565018/rust-clean-code-solution-dynammic-programming/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut dp = vec![0;nums.len()];
        let mut index :i32  = 0 ;
        dp[0]=nums[0]; // we are trying to find max at ith place every time since at 0th position the 0th position itself will be max everytime
        for index in 1..nums.len() {
            if(index==1){
                dp[index]= max(nums[0],nums[1]);
            }else{
                dp[index]= max(dp[index-1],dp[index-2]+nums[index]);
            }

          
        }
     
        return dp[nums.len()-1]
    }

}