// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2817651/rust-always-o-n-even-in-worst-case-scenario-with-comments/
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // since -10^4 <= nums[i] <= 10^4 we will count of occurences of 20000 possible elements
        // -10_000, -9999, ... 0 ... 9999, 10_000
        // bucket[0] represents how many times we incountered -10_000
        // bucket[1] represents how many times we incountered -9999
        // bucket[10000] represents how many times we incountered 0
        // bucket[20000] represents how many times we incountered 10_000
        // bucket[i] represents how many times we incountered i - 10_000
        let mut bucket = [0; 20001];
        // fill bucket according to idea above
        for i in nums.iter() {
            let index = (*i + 10000) as usize;
            bucket[index] += 1;
        }
        // we want to iterate from end of array to its start
        let mut index = 20000_i32;
        // copy k to kk to achive mutability 
        let mut kk = k;
        // if kk is less than zero, it means we left k-th maximum behind
        while kk > 0 {
            kk -= bucket[index as usize];
            index -= 1;
        }
        // substract 9999 as the loop above does one extra iteration 
        // it will point to the number that is 1 lower than k-th max
        index - 9999
    }
}