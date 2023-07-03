// https://leetcode.com/problems/remove-element/solutions/2734992/two-pointers-or-call-api/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // invariant: the slow pointer will point to the last position of values that != val
        // we start from -1 because the 1st element may be equal to `val`
        let mut slow: i32 = -1;
        let mut fast: usize = 0;     

        // if we declare fast as i32, the rust compiler will complain
        while fast < nums.len() {
            if nums[fast] != val {
                slow += 1;
                nums[slow as usize] = nums[fast]
            }

            fast += 1
        }

        (slow + 1) as i32
    }
}