// https://leetcode.com/problems/longest-consecutive-sequence/solutions/3186653/basic-python-solution-beating-92/
class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        max_count = 0
        cur_count = 1
        nums.sort()
        for i in range(len(nums)):
            if (nums[i-1] - nums[i]) == 0:
                pass
            elif (nums[i] - nums[i-1]) == 1:
                cur_count += 1
            else:
                cur_count = 1
            if max_count < cur_count:
                max_count = cur_count
        return max_count

