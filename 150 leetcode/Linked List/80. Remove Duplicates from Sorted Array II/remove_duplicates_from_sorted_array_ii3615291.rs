// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/3615291/rust-two-pointers-one-counter/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

        // should work on any number of maximum consecutive elements
        let MAXCONSEC = 2;
        let n = nums.len();    
        let mut i: usize = 0;  
        let mut j: usize = 1;
        let mut count: usize = 1;

        while j<n {

            let same_value = (nums[i]==nums[j]);
            if (same_value & (count < MAXCONSEC)) | !same_value {

                nums.swap(i+1,j);
                i += 1;
                if !same_value {
                    count = 1;
                } else {
                    count += 1;
                }
            }
            j+=1;
        }
        i += 1;
        return i as i32
    }
}