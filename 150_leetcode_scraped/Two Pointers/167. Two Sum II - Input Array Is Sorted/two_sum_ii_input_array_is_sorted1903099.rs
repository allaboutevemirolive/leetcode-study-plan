// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/1903099/rust-two-loops-solution/
impl Solution {
 pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
  let mut i = 0;
  while i < numbers.len() - 1 {
   let mut j = i + 1;
   while j < numbers.len() {
    let (p1, p2) = (numbers[i], numbers[j]);
    if p1 + p2 == target {
     return vec![(i+1) as i32, (j+1) as i32];
    }
    j += 1;
   }
   i += 1;
  }
 vec![]
 }
}