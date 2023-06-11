// https://leetcode.com/problems/permutations/solutions/2312694/rust-faster-than-100-simple-easy-to-understand-backtracking/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut perms = Vec::new();
        let mut temp  = Vec::new();
        Self::perm(&mut perms, &mut temp, &nums);
        perms
    }
    
    pub fn perm(perms: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>) {
        if temp.len() == nums.len() {
            perms.push(temp.to_vec());
            return;
        }
        
        for n in 0..nums.len() {
            if !temp.contains(&nums[n]) {
                temp.push(nums[n]);
                Self::perm(perms, temp, nums);
                temp.pop();   
            }
        }
    }
}