// https://leetcode.com/problems/remove-element/solutions/3419034/rust-0ms-two-pointers/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        // nums.push(val);
        let mut ret = nums.len();
        let mut j = ret - 1;
        while nums.len() > 0  {
            if nums[j] == val {
                ret -= 1;
                if j == 0 {
                    break;
                }
                j -= 1;
            } else {
                if i >= j {
                    break;
                }
                if nums[i] == val {
                    nums[i] = nums[j];
                    nums[j] = val;
                    i += 1;
                } else {
                    i += 1;
                }
            }
        }
        (ret) as i32

    }
}