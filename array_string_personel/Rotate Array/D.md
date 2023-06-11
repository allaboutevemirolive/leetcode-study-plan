impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let real_k: usize = k as usize % nums.len();
        Solution::rotate_within_range(nums, 0, nums.len() - 1);
        Solution::rotate_within_range(nums, 0, real_k - 1);
        Solution::rotate_within_range(nums, real_k, nums.len() -1);
    }
    
    fn rotate_within_range(nums: &mut Vec<i32>, start: usize, end: usize) {
        if end >= nums.len() {
            return;
        }
        
        let mut s = start;
        let mut e = end;
        
        while s < e {
            nums.swap(s, e);
            s += 1;
            e -= 1;
        }
    }
}