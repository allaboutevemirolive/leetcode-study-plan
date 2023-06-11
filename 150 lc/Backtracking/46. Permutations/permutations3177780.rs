// https://leetcode.com/problems/permutations/solutions/3177780/rust-0ms-runtime-beats-100/
impl Solution {
    fn combine_and_push(
        remaining: i32,
        nums: &[i32],
        combination: Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
        used_indices: &mut Vec<usize>,
    ) {
        if remaining == 0 {
            combinations.push(combination);
        } else {
            for i in 0..nums.len() {
                if used_indices.contains(&i) {
                    continue;
                }
                used_indices.push(i);
                let mut combination = combination.clone();
                combination.push(nums[i]);
                Self::combine_and_push(
                    remaining - 1,
                    nums,
                    combination,
                    combinations,
                    used_indices,
                );
                used_indices.pop();
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combinations = Vec::new();
        Self::combine_and_push(
            nums.len() as i32,
            &nums,
            Vec::new(),
            &mut combinations,
            &mut Vec::new(),
        );
        combinations
    }
}
