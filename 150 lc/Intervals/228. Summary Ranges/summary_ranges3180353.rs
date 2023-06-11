// https://leetcode.com/problems/summary-ranges/solutions/3180353/rust-simple-solution/
impl Solution {
    fn format_range(start: i32, end: i32) -> String {
        if start == end {
            format!("{start}")
        } else {
            format!("{start}->{end}")
        }
    } 
    
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {      
        match nums.len() {
            0 => vec![],
            1 => vec![nums[0].to_string()],
            _ => {
                let mut res: Vec<String> = Vec::new();
                let mut start = nums[0];

                for i in 1..nums.len() {
                    if nums[i - 1] + 1 != nums[i] {
                        res.push(Self::format_range(start, nums[i - 1]));
                        start = nums[i];
                    }
                }
                res.push(Self::format_range(start, nums[nums.len() - 1]));
                res
            }
        }
    }
}