189. Rotate Array
Medium
14.4K
1.6K
Companies
Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

 

Example 1:

Input: nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
Explanation:
rotate 1 steps to the right: [7,1,2,3,4,5,6]
rotate 2 steps to the right: [6,7,1,2,3,4,5]
rotate 3 steps to the right: [5,6,7,1,2,3,4]
Example 2:

Input: nums = [-1,-100,3,99], k = 2
Output: [3,99,-1,-100]
Explanation: 
rotate 1 steps to the right: [99,-1,-100,3]
rotate 2 steps to the right: [3,99,-1,-100]
 

Constraints:

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1
0 <= k <= 105


___



The task is to rotate an array called `nums` to the right by `k` steps.

Here's a simpler explanation:

You are given an array of numbers called `nums` and a non-negative integer `k`. Your goal is to rotate the array to the right by `k` steps.

To rotate an array to the right means to move the last `k` elements of the array to the front, while shifting the remaining elements to the right. The order of the elements should be preserved.

To solve this, you can perform the following steps:

1. Determine the actual number of steps needed for rotation. If `k` is larger than the length of the array, take the modulo (`k % length`) to get the effective number of steps.

2. Create a new array of the same length as `nums` and copy the elements from `nums` to the new array.

3. Iterate over the new array. For each element at index `i`, calculate the new index after rotation using the formula `(i + k) % length`, where `length` is the length of the array. Assign the element at the new index to the corresponding position in the original `nums` array.

4. The resulting `nums` array will be the rotated array.

Return the rotated `nums` array as the output of the function.

Consider the given constraints while solving the problem. The length of the `nums` array can be up to 10^5, and the values in the array are between -2^31 and 2^31 - 1. The value of `k` can be up to 10^5.