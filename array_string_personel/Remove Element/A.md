27. Remove Element

Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
Return k.
Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int val = ...; // Value to remove
int[] expectedNums = [...]; // The expected answer with correct length.
                            // It is sorted with no values equaling val.

int k = removeElement(nums, val); // Calls your implementation

assert k == expectedNums.length;
sort(nums, 0, k); // Sort the first k elements of nums
for (int i = 0; i < actualLength; i++) {
    assert nums[i] == expectedNums[i];
}
If all assertions pass, then your solution will be accepted.

 

Example 1:

Input: nums = [3,2,2,3], val = 3
Output: 2, nums = [2,2,_,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 2.
It does not matter what you leave beyond the returned k (hence they are underscores).


Example 2:

Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5, nums = [0,1,4,0,3,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
Note that the five elements can be returned in any order.
It does not matter what you leave beyond the returned k (hence they are underscores).
 

Constraints:

0 <= nums.length <= 100
0 <= nums[i] <= 50
0 <= val <= 100

___


The task is to remove all occurrences of a given value, `val`, from an array called `nums`. After removing the elements, the function should return the number of remaining elements in the array.

Here's a simpler explanation:

You are given an array of numbers called `nums` and a specific number called `val`. Your goal is to remove all occurrences of `val` from the array and count the number of elements that remain.

To solve this, you need to iterate through the array and check each element. If the element is equal to `val`, you should remove it from the array. To remove an element from the array, you can shift all the elements to the left starting from that element's position.

After removing all occurrences of `val`, the remaining elements will be placed at the beginning of the array. The length of the array will be the count of the remaining elements.

Return this count as the output of the function.

Note that the order of the elements in the array can change after removing the values, but it doesn't matter as long as the count of the remaining elements is correct.

Consider the given constraints while solving the problem. The length of the `nums` array can be up to 100, and the values in the array and `val` are between 0 and 100.