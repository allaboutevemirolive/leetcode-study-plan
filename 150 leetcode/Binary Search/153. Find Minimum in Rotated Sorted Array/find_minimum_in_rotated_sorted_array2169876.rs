// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/2169876/rust-solution-100-faster-time-complexity-o-logn/
impl Solution {
 pub fn find_min(nums: Vec<i32>) -> i32 {
  let (front, last) = (nums[0], nums[nums.len() - 1]);
  if front > last {
   let mut i = nums.len() - 1;
   let mut next = nums[i - 1];
   let mut min = last;
   while next <= min {
    min = next;
    next = nums[i - 1];
    i -= 1;
   }
   return min;
  } else {
   return front;
  }
 }
}