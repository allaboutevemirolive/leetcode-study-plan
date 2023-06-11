// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/3099857/rust-binary-search-on-rotated-slice/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        #[inline(always)]
        fn is_rotated(num_slice: &[i32]) -> bool {
            num_slice.first() > num_slice.last()
        }
        let (mut l, mut r) = (0, nums.len()-1);
        if !is_rotated(&nums[..]) {
            return nums[0];
        }
        while l <= r {
            let m = l + (r - l) / 2;
            if m != (nums.len()-1) && nums[m] > nums[m+1] {
                return nums[m+1];
            }
            match is_rotated(&nums[l..=m]) {
                true => r = m,
                false => l = m+1,
            }
        }
        unreachable!()
    }
}