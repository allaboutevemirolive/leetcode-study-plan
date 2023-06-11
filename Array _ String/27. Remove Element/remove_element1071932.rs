// https://leetcode.com/problems/remove-element/solutions/1071932/rust-solution/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        
        let mut i = 0;
        let mut max = nums.len();
        
        while i < max{ 
            if nums[i] == val {
                nums.remove(i);
                max -= 1;
                i -= 1;
            }
            i += 1;
        }
        nums.len() as i32
    }
}