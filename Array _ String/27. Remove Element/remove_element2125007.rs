// https://leetcode.com/problems/remove-element/solutions/2125007/rust/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        } else if nums.len() == 1 {
            if nums[0] == val {
                return 0;
            }
        }
        let mut i = 0;
        let mut len = (nums.len() - 1) as i32;
        'a: while (i <= len) {
            if nums[i as usize] == val {
                'b: while (len >= i) {
                    if nums[len as usize] != val {
                        nums[i as usize] = nums[len as usize];
                        len -= 1;
                        break 'b;
                    } else {
                        len -= 1;
                    }
                }
            }

            i += 1;
        }

        len + 1
    }
}