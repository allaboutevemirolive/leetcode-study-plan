// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3460937/avoid-reinventing-the-wheel-by-using-partition-point-and-binary-search/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let split = nums.partition_point(|&x| x >= nums[0]);
        if target < nums[0] {
            if let Ok(i) = nums[split..].binary_search(&target) {
                (i + split) as i32
            } else {
                -1
            }
        } else {
            if let Ok(i) = nums[..split].binary_search(&target) {
                i as i32
            } else {
                -1
            }
        }
    }
}