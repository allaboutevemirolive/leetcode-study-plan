// https://leetcode.com/problems/remove-element/solutions/3183107/rust-with-reverse-on-updates/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let  mut updates:Vec<usize> = Vec::new();
        for (count,num) in nums.iter().enumerate() {
            if val == *num {
                updates.push(count);
            }
        }

        let mut last = nums.len() -1;
        //must move in reverse order
        updates = updates.into_iter().rev().collect();

        for loc in updates {
            nums[loc] = nums[last];
            last = last - 1;
        }
        last as i32 + 1
        
    }
}