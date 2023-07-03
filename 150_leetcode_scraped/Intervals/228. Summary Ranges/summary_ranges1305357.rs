// https://leetcode.com/problems/summary-ranges/solutions/1305357/rust-easy-solution-0ms/
impl Solution {
  pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
      return Vec::new();
    }
    let mut start:i32 = nums[0];
    let mut end:i32 = nums[0]+1;
    let mut res:Vec<String> = Vec::new();
    for i in 1..nums.len() {
      if nums[i] == end {
        end += 1;
      } 
      else {
        if end - start == 1 {
          res.push(format!("{}",start));
        } else {
          res.push(format!("{}->{}",start,nums[i-1]));
        }
        start = nums[i];
        end = nums[i]+1;
      }
    }
    if start == nums[nums.len()-1] {
      res.push(format!("{}",nums[nums.len()-1]))
    } else {
      res.push(format!("{}->{}",start,nums[nums.len()-1]))
    }
    return res;
  }
}