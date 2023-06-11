// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3043438/binary-search-o-log-n/
class Solution {
    public int firstPosition(int[] nums, int target) {
        int low = 0;
        int high = nums.length - 1;

        while (low <= high) {
            int mid = low + (high - low) / 2;

            if (nums[mid] > target) {
                high = mid - 1;
            }
            else if (nums[mid] < target) {
                low = mid + 1;
            }
            else {
                if (mid == 0 || nums[mid - 1] != nums[mid]) {
                    return mid;
                }
                else {
                    high = mid - 1;
                }
            }
        }

        return -1;
    }

    public int lastPosition(int[] nums, int target) {
        int low = 0;
        int high = nums.length - 1;

        while (low <= high) {
            int mid = low + (high - low) / 2;

            if (nums[mid] > target) {
                high = mid - 1;
            }
            else if (nums[mid] < target) {
                low = mid + 1;
            }
            else {
                if (mid == nums.length - 1 || nums[mid + 1] != nums[mid]) {
                    return mid;
                }
                else {
                    low = mid + 1;
                }
            }
        }

        return -1;
    }

    public int[] searchRange(int[] nums, int target) {
        return new int[]{firstPosition(nums, target), lastPosition(nums, target)};
    }
}