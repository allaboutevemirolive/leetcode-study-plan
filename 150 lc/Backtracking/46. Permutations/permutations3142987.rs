// https://leetcode.com/problems/permutations/solutions/3142987/rust-2-ms-2-3mb-backtracking/
use std::collections::HashSet;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        /*
            1. Generate the base case
            2. Do everything to meet the conditions
        */
        
        let mut res: Vec<Vec<i32>> = vec![];
        let mut curr: Vec<i32> = vec![];
        let mut visited = HashSet::new();
        Self::generate_nums(&nums, &mut curr, &mut res, &mut visited);
        
        
        res
        
    }
    
    fn generate_nums(
        nums: &Vec<i32>, 
        mut curr: &mut Vec<i32>,
        mut res: &mut Vec<Vec<i32>>,
        mut visited: &mut HashSet<i32>
    ) {
        
        if curr.len() >= nums.len() { 
            res.push(curr.to_vec());
            return;
        }
        for next_node in 0..nums.len() { 
            let val = nums[next_node];
            if visited.contains(&val) { continue }
            visited.insert(val);
            curr.push(val);            
			
            Self::generate_nums(nums, curr, res, visited);
            // Backtrack
			curr.pop();
            visited.remove(&val);
        }
    }
}