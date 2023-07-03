// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3363519/rust-solution-1-more/
impl Solution
{
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32
    {
        let mut curr: (i32, usize) = (nums[0], 1);

        for i in 1..nums.len()
        {
            if nums[i] > curr.0
            {
                nums.swap(i, curr.1);
                curr = (nums[curr.1], curr.1 + 1);
            }
        }

        curr.1 as i32
    }
}