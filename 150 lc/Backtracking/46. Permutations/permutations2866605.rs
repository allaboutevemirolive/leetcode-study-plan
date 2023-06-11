// https://leetcode.com/problems/permutations/solutions/2866605/a-rust-solution-using-openai-to-generate-the-code/
use std::collections::VecDeque;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Initialize a list to store the permutations
        let mut permutations = Vec::new();

        // Define a recursive function to find the permutations of the given list
        fn helper(nums: &Vec<i32>, current: Vec<i32>, permutations: &mut Vec<Vec<i32>>) {
            // If the current list is empty, append the current permutation to the list of permutations
            if nums.is_empty() {
                permutations.push(current);
                return;
            }

            // Iterate over the elements in the current list
            for (i, num) in nums.iter().enumerate() {
                // Create a new list with the current element removed
                let mut new_nums = nums.to_vec();
                new_nums.remove(i);

                // Find the permutations of the new list and append the current element to each permutation
                let mut new_current = current.to_vec();
                new_current.push(*num);
                helper(&new_nums, new_current, permutations);
            }
        }

        // Find the permutations of the given list
        helper(&nums, Vec::new(), &mut permutations);

        // Return the list of permutations
        permutations
    }
}