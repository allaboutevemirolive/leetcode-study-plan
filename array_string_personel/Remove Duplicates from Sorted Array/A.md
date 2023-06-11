26. Remove Duplicates from Sorted Array


Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
Return k.
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

Input: nums = [1,1,2]
Output: 2, nums = [1,2,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).


Example 2:

Input: nums = [0,0,1,1,1,2,2,3,3,4]
Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).
 

Constraints:

1 <= nums.length <= 3 * 104
-100 <= nums[i] <= 100
nums is sorted in non-decreasing order.


___


The task is to remove duplicates from a sorted array called `nums` and return the number of unique elements remaining. The relative order of the elements should be preserved.

Here's a simpler explanation:

You are given an array of numbers called `nums`, which is sorted in non-decreasing order. Your goal is to remove any duplicate numbers from the array, keeping only the unique elements. The order of the elements should remain the same.

To solve this, you need to iterate through the array and compare each element with the previous one. If the current element is equal to the previous element, it means it's a duplicate and should be removed. To remove an element from the array, you can shift all the elements to the left starting from that element's position.

After removing all duplicates, the remaining elements will be placed at the beginning of the array. The length of the array will be the count of the unique elements.

Return this count as the output of the function.

Note that the order of the elements should be maintained, which means if an element appears earlier in the array, it should also appear earlier in the resulting array. The remaining elements after the unique elements are not important, and their values can be anything (often represented as underscores in examples).

Consider the given constraints while solving the problem. The length of the `nums` array can be up to 3 * 10^4, and the values in the array are between -100 and 100.