// https://leetcode.com/problems/remove-element/solutions/1831547/rust-solution/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  let mut k: i32 = 0;
  
  for i in 0..nums.len() {
    if nums[i] != val {
      nums[k as usize] = nums[i];
      k += 1;
    }
  }

  k
}