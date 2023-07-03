// https://leetcode.com/problems/summary-ranges/solutions/1272966/rust-using-format/
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut previous = nums[0]-1;
        let mut start = nums[0];
        let mut result = vec![];
        for i in 0..nums.len() {
            if nums[i] != (previous+1) {
                if start == nums[i-1] {
                    result.push(format!("{}", start));
                }
                else {
                    result.push(format!("{}->{}", start, nums[i-1]));
                }
                start = nums[i];
            }
            previous = nums[i];
        }
        
        if start == nums[nums.len()-1] {
            result.push(format!("{}", start));   
        }
        else {
            result.push(format!("{}->{}", start, nums[nums.len()-1]));
        }
        result
    }
}