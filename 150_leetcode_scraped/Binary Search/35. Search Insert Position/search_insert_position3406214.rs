// https://leetcode.com/problems/search-insert-position/solutions/3406214/rust-solution-0ms/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target <= nums[0] {
            return 0;
        };
        if target > nums[nums.len() - 1] {
            return nums.len() as i32;
        };

        let mut res: i32 = 0;

        for (idx, item) in nums.iter().enumerate() {
            if (item == &target) || (idx > 0 && target > nums[idx - 1] && target < *item) {
                res = idx as i32;
                break;
            }
        };

        return res;
    }
}