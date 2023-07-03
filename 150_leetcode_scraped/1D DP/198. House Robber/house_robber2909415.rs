// https://leetcode.com/problems/house-robber/solutions/2909415/rust-0ms-o-1-aux-space-clean/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut back_1 = 0;
        let mut back_2 = 0;

        for i in 0..nums.len() {
            let choice_1 = nums[i] + back_2;
            let choice_2 = back_1;
            let max      = choice_1.max(choice_2);

            back_2 = back_1;
            back_1 = max;
        }
        back_1
    }
}