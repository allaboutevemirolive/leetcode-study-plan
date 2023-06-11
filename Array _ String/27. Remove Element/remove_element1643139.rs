// https://leetcode.com/problems/remove-element/solutions/1643139/rust-swap-remove/
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.swap_remove(i);
                continue;
            }
            
            i += 1;
        }
        
        nums.len() as i32
    }