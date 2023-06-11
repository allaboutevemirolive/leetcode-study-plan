80. Remove Duplicates from Sorted Array II
Medium
5.1K
1K
Companies
Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

Return k after placing the final result in the first k slots of nums.

Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
If all assertions pass, then your solution will be accepted.

 

Example 1:

Input: nums = [1,1,1,2,2,3]
Output: 5, nums = [1,1,2,2,3,_]
Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).


Example 2:

Input: nums = [0,0,1,1,1,1,2,3,3]
Output: 7, nums = [0,0,1,1,2,3,3,_,_]
Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).
 

Constraints:

1 <= nums.length <= 3 * 104
-104 <= nums[i] <= 104
nums is sorted in non-decreasing order.


___


The task is to remove duplicates from a sorted array called `nums`, but with a twist: each unique element can appear at most twice in the resulting array. The relative order of the elements should be preserved.

Here's a simpler explanation:

You are given an array of numbers called `nums`, which is sorted in non-decreasing order. Your goal is to modify the array in-place to remove duplicates. However, each unique element can appear at most twice in the modified array.

To solve this, you need to iterate through the array and compare each element with the previous two elements. If the current element is equal to the previous two elements, it means it's a duplicate and should be removed. To remove an element from the array, you can shift all the elements to the left starting from that element's position.

After removing duplicates, the remaining elements will be placed at the beginning of the array. The length of the array will be the count of the unique elements.

Return this count as the output of the function.

Note that the order of the elements should be maintained, which means if an element appears earlier in the array, it should also appear earlier in the resulting array. The remaining elements after the unique elements are not important, and their values can be anything (often represented as underscores in examples).

You should modify the input array in-place and not allocate extra space for another array. You can assume that you have O(1) extra memory available.

Consider the given constraints while solving the problem. The length of the `nums` array can be up to 3 * 10^4, and the values in the array are between -10^4 and 10^4.