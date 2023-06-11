// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3531869/peakable-iterators-rust-solution-with-5-total-approaches/
impl Solution {
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut new: Vec<i32> = Vec::new();
    let mut iterator = nums.iter().peekable();
    while let Some(num) = iterator.next() {
        match iterator.peek() {
            Some(n) => {
                if *n != num {
                    new.push(*num);
                }
            }
            None => {
                new.push(*num);
            }
        }
    }
    *nums = new;
    nums.len() as i32
}
}