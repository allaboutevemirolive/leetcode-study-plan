// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3532883/100-time-rust-python-js-learn-rust-with-me/
Rust 0ms
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut ptr = 0;
        for i in 1..nums.len() {
            if(nums[i] != nums[ptr]) {
                nums[ptr+1] = nums[i];
                ptr+=1;
            }
        }
        (ptr+1 ) as i32
    }
}

Python 104ms
def removeDuplicates(self, nums: List[int]) -> int:
        ptr = 0
        for i in range(1, len(nums)):
            if nums[i] != nums[ptr]:
                nums[ptr + 1] = nums[i]
                ptr += 1
        return ptr + 1

Javascript 73ms
function removeDuplicates(nums) {
    let ptr = 0;
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] !== nums[ptr]) {
            nums[ptr + 1] = nums[i];
            ptr++;
        }
    }
    return ptr + 1;
}
