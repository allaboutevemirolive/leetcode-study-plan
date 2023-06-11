// https://leetcode.com/problems/maximum-subarray/solutions/2824407/o-n-solution-in-python-using-kadane-s-algorithm/
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        maxSum = nums[0] 
        curSum = 0 
        for num in nums:
            if (curSum) < 0:
                curSum = num 
            else:
                curSum = curSum + num 
            maxSum = max(maxSum, curSum)
        return maxSum