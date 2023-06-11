// https://leetcode.com/problems/remove-element/solutions/3445201/rust-solution-using-peekable-iterator/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;
        let mut iter = (0..nums.len()).into_iter().peekable();
        while let Some(&i) = iter.peek() {
            if nums[i] == val {
                let mut idx = i;
                while idx < nums.len() - 1 {
                    nums.swap(idx, idx + 1);
                    idx += 1;
                }
                if i == nums.len() - 1 || i + count == nums.len() {
                    if count == 0 {
                        count += 1;
                    }
                    break;
                }
                count += 1;
            } else {
                iter.next();
            }
        }
        (nums.len() - count) as i32
    }
}
