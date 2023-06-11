// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3478736/rust-simple-solution/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut position: (i32, i32) = (-1, -1);
        let mut first_position = 0;
        for (i, num) in nums.iter().enumerate() {
            if num == &target {
                position.0 = i as i32;
                first_position = i;
                break;
            }
        }
        if position.0 == -1 {
            return vec!(-1, -1);
        }
        for i in first_position+1..nums.len() {
            if nums.get(i).is_some() && nums.get(i).unwrap() > &target {
                position.1 = i as i32-1;
                break;
            }
        }
        if position.1 == -1 {
            return vec!(position.0, nums.len() as i32-1);
        }
        vec!(position.0, position.1)
    }
}