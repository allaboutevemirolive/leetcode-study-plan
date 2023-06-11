// https://leetcode.com/problems/merge-sorted-array/solutions/3001406/a-recursive-solution-in-rust/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
        // If we were allowed to change the signature of `merge`, we wouldn't
        // need a helper function and could do everything here.
        merge_recursively(nums1, nums2, m as usize);
    }
}

fn merge_recursively(nums1: &mut [i32], nums2: &mut [i32], nums_left_in_1: usize) {
    if nums_left_in_1 == 0 {
        // Just "paste" what remains of `nums2` in the space reserved for it in
        // `nums1`.
        nums1.swap_with_slice(nums2);
    } else if let (Some(&num1), Some(&num2)) = (nums1.first(), nums2.first()) {
        if num1 <= num2 {
            // Don't touch `nums1`; "advance" by 1 by calling the function on
            // a sub-slice of `nums1` that excludes the first element.
            merge_recursively(&mut nums1[1..], nums2, nums_left_in_1 - 1);
        } else {
            // Do touch `nums1`; move all of its numbers forward by 1 and then
            // insert `num2`—the smaller of the two—as its first element.
            // Then "advance" both lists by 1 the same as above. `nums_left_in_1`
            // remains unchanged.
            scooch_forward(nums1, nums_left_in_1);
            nums1[0] = num2;
            merge_recursively(&mut nums1[1..], &mut nums2[1..], nums_left_in_1);
        }
    }
}

fn scooch_forward(nums: &mut [i32], mut nums_left: usize) {
    // This could be replaced with a `for` loop, but I think
    // it's more readable this way.
    while nums_left > 0 {
        nums.swap(nums_left, nums_left - 1);
        nums_left -= 1;
    }
}