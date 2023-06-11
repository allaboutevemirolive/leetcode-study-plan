45. Jump Game II
Medium
12.4K
434
Companies
You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].

Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:

0 <= j <= nums[i] and
i + j < n
Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

 

Example 1:

Input: nums = [2,3,1,1,4]
Output: 2
Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
Example 2:

Input: nums = [2,3,0,1,4]
Output: 2
 

Constraints:

1 <= nums.length <= 104
0 <= nums[i] <= 1000
It's guaranteed that you can reach nums[n - 1].

___


The given problem is about finding the minimum number of jumps required to reach the last index of an array, where each element represents the maximum length of a forward jump from that index.

Here's a simplified explanation of the problem:

- You are given an array of integers called `nums`.
- You start at the first element of the array, `nums[0]`.
- Each element `nums[i]` represents the maximum number of steps you can jump forward from index `i`.
- The goal is to reach the last index of the array, `nums[n - 1]`, using the minimum number of jumps.

For example:
- If `nums = [2, 3, 1, 1, 4]`, the minimum number of jumps required to reach the last index is 2. You can jump 1 step from index 0 to index 1, and then 3 steps to the last index.
- If `nums = [2, 3, 0, 1, 4]`, the minimum number of jumps required to reach the last index is also 2. You can jump 2 steps from index 0 to index 2, and then 2 steps to the last index.

The task is to determine the minimum number of jumps needed to reach the last index in the given array `nums`. The provided test cases guarantee that it is possible to reach the last index.

Constraints:
- The length of the array `nums` is between 1 and 10^4.
- The values in `nums` are non-negative integers between 0 and 1000.