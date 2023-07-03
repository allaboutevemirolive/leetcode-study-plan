// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2543510/rust-sort-and-iteration-runtime-18ms-memory-usage-3-3mb/
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        
        let mut acc = i32::MIN;
        
        let mut tmp = 0;
        let mut res = 0;

        for val in nums {
            if val<acc {continue;}
            if val==acc {
                tmp += 1;
                acc += 1;
            } else {
                acc = val+1;
                res = res.max(tmp);
                tmp = 1;
            }
        }
        res.max(tmp)
    }
}