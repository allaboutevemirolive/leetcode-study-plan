// https://leetcode.com/problems/combination-sum/solutions/3573206/learning-rust-dereferencing-and-moves/
fn DFS(target: &mut i32, candidates: &Vec<i32>, pot_combination: &mut Vec<i32>, all_combinations: &mut Vec<Vec<i32>>, idx: usize, nums_length: &usize) {
    if *target < 0 {
        return;
    } else if *target == 0 {
        all_combinations.push(pot_combination.to_vec());
        return;
    }

    for curr_idx in idx..*nums_length {
        pot_combination.push(candidates[curr_idx]);
        *target -= candidates[curr_idx];

        DFS(target, candidates, pot_combination, all_combinations, curr_idx, nums_length);

        *target += candidates[curr_idx];
        pot_combination.pop();
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums_length = candidates.len();
        let mut target_copy = target;
        let mut pot_combination = vec![];
        let mut all_combinations = vec![];

        DFS(&mut target_copy, &candidates, &mut pot_combination, &mut all_combinations, 0, &nums_length);
        all_combinations
        
    }
}