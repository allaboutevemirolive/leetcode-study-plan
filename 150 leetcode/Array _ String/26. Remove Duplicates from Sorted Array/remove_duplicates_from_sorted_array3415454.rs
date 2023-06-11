// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3415454/solution-in-rust-in-3-lines/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.sort();                    // sort list
        nums.dedup();                   // remove duplicates from sorted list
        return nums.len() as i32;       // return number of elements in the list
    }
}