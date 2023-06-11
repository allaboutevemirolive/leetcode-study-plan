// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3360452/idiomatic-rust/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut previous = None;
        let mut k = 0;

        (0..nums.len()).for_each(|i|{
            if Some(nums[i]) != previous {
                previous = Some(nums[i]);
                nums.swap(i, k);
                k += 1;
            }
        });

        k as i32
    }
}