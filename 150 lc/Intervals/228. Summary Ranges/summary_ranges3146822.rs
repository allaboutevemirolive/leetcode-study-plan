// https://leetcode.com/problems/summary-ranges/solutions/3146822/rust-simple-solution/
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut ans = vec![];
        let mut interval = (nums[0], nums[0]);
        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] == 1 {
                interval.1 = nums[i];
            } else {
                ans.push(interval);
                interval = (nums[i], nums[i]);
            }
        }
        ans.push(interval);
        ans.into_iter()
            .map(|interval| {
                if interval.0 == interval.1 {
                    format!("{}", interval.0)
                } else {
                    format!("{}->{}", interval.0, interval.1)
                }
            })
            .collect()  
    }
}