// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3023772/rust-100/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    let mut write: usize = 0; 
    let mut  read: usize = 1; 

    loop {

        if read == nums.len() { return write as i32 + 1; }

        if nums[write] != nums[read] {

            write += 1;
            nums[write] = nums[read]
        }

        read += 1;
    }
}
