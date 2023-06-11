55. Jump Game
Medium
16.3K
849
Companies
You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.

 

Example 1:

Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
Example 2:

Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
 

Constraints:

1 <= nums.length <= 104
0 <= nums[i] <= 105

___


The task is to determine whether it is possible to reach the last index of an array by jumping based on the maximum jump length given at each position.

Here's a simpler explanation:

You are given an array called `nums`, where each element represents the maximum jump length you can take from that position. Starting from the first index of the array, your goal is to reach the last index of the array.

To solve this problem, you can use a greedy approach. Iterate over the elements of the array and keep track of the maximum index you can reach. At each position, update the maximum index by taking the maximum between the current maximum index and the index you can reach from the current position.

If, at any point, the maximum index surpasses or reaches the last index of the array, return `true` because it means you can reach the last index.

If the iteration finishes and the maximum index is still less than the last index, return `false` because it means you cannot reach the last index.

For example, consider the array `[2, 3, 1, 1, 4]`:

- Start at index 0 and update the maximum index to 2 (jumping 2 steps).
- Move to index 1 and update the maximum index to 4 (jumping 3 steps).
- Move to index 2 and update the maximum index to 4 (jumping 1 step).
- Move to index 3 and update the maximum index to 4 (jumping 1 step).
- Finally, reach the last index, which is 4.

Since you were able to reach the last index, return `true`.

Consider the given constraints while solving the problem. The length of the `nums` array can be up to 10^4, and the values in the array are between 0 and 10^5.