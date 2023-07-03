// https://leetcode.com/problems/maximum-subarray/solutions/2976513/fast-rust-kadane-s-algo-with-two-implementations/
impl Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        // Initialize the maximum sum to the minimum possible value for i32
        let mut max_sum = std::i32::MIN;

        // Initialize the current sum to 0
        let mut current_sum = 0;

        // Iterate through the elements of the input array
        for &num in nums.iter() {
            // Update the current sum by adding the current element
            // If the current sum is negative, set it to 0 instead
            current_sum = current_sum.max(0) + num;

            // Update the maximum sum if necessary
            max_sum = max_sum.max(current_sum);
        }

        // If the maximum sum is still the minimum possible value, the input array
        // consists only of negative numbers or is empty. In this case, return the
        // maximum element in the array, or 0 if the array is empty.
        if max_sum == std::i32::MIN {
            return *nums.iter().max().unwrap_or(&0);
        }

        // Return the maximum sum
        max_sum
    }
}