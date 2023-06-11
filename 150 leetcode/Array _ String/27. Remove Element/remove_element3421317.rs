// https://leetcode.com/problems/remove-element/solutions/3421317/rust-concise/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ret = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                    nums[ret] = nums[i];
                    ret += 1;
            }
        }
        // println!("{:?}", nums);
        ret as i32
    }
}