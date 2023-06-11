// https://leetcode.com/problems/permutations/solutions/2873101/iterative-rust-solution/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut result = vec![nums.clone()];

        loop {
            if Solution::next_permutation(&mut nums) {
                result.push(nums.clone());
            } else {
                break;
            }
        }

        result
    }

    fn next_permutation(nums: &mut Vec<i32>) -> bool {
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        let mut j = nums.len() - 1;
        while nums[j] <= nums[i - 1] {
            j -= 1;
        }

        nums.swap(i - 1, j);

        nums[i..].reverse();

        true
    }
}