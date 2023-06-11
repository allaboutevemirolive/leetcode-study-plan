// https://leetcode.com/problems/summary-ranges/solutions/2201635/100-0ms-one-loop-o-n-rust/
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut range = (None, None);
        let mut next_num: i32;
        
        for (index, num) in nums.iter().enumerate() {
            range.0 = match range.0 {
                Some(x) => Some(x),
                None => Some(num),
            };
            
            if index == nums.len() -1 || num + 1 != nums[index + 1] {
                let to_push = match range.1 {
                    Some(x) => format!("{}->{}", range.0.unwrap(), range.1.unwrap()),
                    None => format!("{}",range.0.unwrap()), 
                };
                
                result.push(to_push);
                range = (None, None);
            } else {
                range.1 = Some(num + 1);
            }
        }
        result
    }
}