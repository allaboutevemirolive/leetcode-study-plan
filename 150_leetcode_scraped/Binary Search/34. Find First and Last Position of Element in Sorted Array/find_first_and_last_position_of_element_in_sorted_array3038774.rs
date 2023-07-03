// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3038774/python3-simple-binary-search-solution-o-logn/
class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        starting = -1
        ending = -1

        return self.starting(nums, target, starting), self.ending(nums, target, ending)

    def starting(self, nums, target, starting):
            low = 0
            high = len(nums)-1
            while low<=high:
                mid = low+(high-low)//2
                if target == nums[mid]:
                    starting = mid
                    high = mid-1
                elif nums[mid]<target:
                    low = mid+1
                else:
                    high = mid-1
            return starting
                    
    def ending(self, nums, target, ending):
        low = 0
        high = len(nums)-1
        while low<=high:
            mid = low+(high-low)//2
            if target == nums[mid]:
                ending = mid
                low = mid+1
            elif nums[mid]<target:
                low = mid+1
            else:
                high = mid-1
        return ending
