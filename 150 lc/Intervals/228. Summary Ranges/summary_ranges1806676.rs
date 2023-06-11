// https://leetcode.com/problems/summary-ranges/solutions/1806676/rust-solution/
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        
        if nums.is_empty() {
            return result;
        }

        let get_str_range = |start: i32, end: i32| -> String {
            match start == end {
                true => format!("{}", start),
                false => format!("{}->{}", start, end),
            }
        };

        let mut start = nums[0];
        let mut end = start;

        for &x in &nums[1..] {
            if x > end + 1 {
                result.push(get_str_range(start, end));
                start = x;
            }
            end = x
        }
        
        result.push(get_str_range(start, end));
        result
    }
}