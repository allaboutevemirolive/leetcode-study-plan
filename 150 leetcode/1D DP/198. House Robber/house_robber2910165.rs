// https://leetcode.com/problems/house-robber/solutions/2910165/python-c-rust-iterative-recursive-solutions-explained-bonus-one-liner/
class Solution:
    
    def rob(self, nums):
        
        @cache
        def dfs(i):
            return max(dfs(i+1), nums[i] + dfs(i+2)) if i < len(nums) else 0
        
        return dfs(0)