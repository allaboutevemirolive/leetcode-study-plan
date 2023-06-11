// https://leetcode.com/problems/permutations/solutions/3225894/super-duper-simple-rust-backtracing/
impl Solution {
    pub fn helper(mut nums:Vec<i32>, mut path:Vec<i32>, mut res: &mut Vec<Vec<i32>>){
        if nums.is_empty() {
            res.push(path);
            return;
        }
        for i in 0..nums.len() {
            let mut nums_copy = nums.clone();
            let mut path_copy = path.clone();
            let num = nums_copy.remove(i);
            path_copy.push(num);
            Self::helper(nums_copy, path_copy, res);
        }
        
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res:Vec<Vec<i32>> = Vec::new();
        Self::helper(nums, Vec::new(), &mut res); 
        return res;
    }
}