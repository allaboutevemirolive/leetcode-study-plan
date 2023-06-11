// https://leetcode.com/problems/summary-ranges/solutions/2794438/rust-simple-iteration/
impl Solution {
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    fn answ_format(from: i32, to: i32) -> String {
        if from == to {
            format!("{}", from)
        } else {
            format!("{}->{}", from, to)
        }
    }

    if nums.is_empty() {
        Vec::new()
    } else {
        let (mut from, mut to) = (nums[0], nums[0]);
        let mut answ = Vec::new();
        for i in 1..nums.len() {
           if to+1 != nums[i] {
            answ.push(answ_format(from, to));
            from = nums[i];
            to = nums[i];
           } else {
            to+=1
           } 
        }
        answ.push(answ_format(from, to));
        answ
    }
}
}