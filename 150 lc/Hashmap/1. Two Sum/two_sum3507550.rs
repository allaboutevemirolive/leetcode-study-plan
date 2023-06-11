// https://leetcode.com/problems/two-sum/solutions/3507550/rust-tc-o-n-2-sc-o-1/
impl Solution {
    impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        // Iterate through the array using enumerate to keep track of the index and value
        for (i, val) in nums.iter().enumerate() {
            
            // Compute the remaining value needed to reach the target
            let remaining_val = target - val;
            
            // Iterate through the remaining values in the array starting from the next index
            for index2 in i + 1..nums.len() {
                
                // If a match is found, return a vector of the current index and the index of the matching value
                if remaining_val == nums[index2] {
                    return vec![i as i32, index2 as i32];
                }
            }
        }
        
        // If no match is found, return a vector of two zeros
        return vec![0; 2];
    }
}

}