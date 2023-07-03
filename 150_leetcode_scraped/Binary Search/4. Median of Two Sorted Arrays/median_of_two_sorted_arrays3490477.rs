// https://leetcode.com/problems/median-of-two-sorted-arrays/solutions/3490477/rust-4-line-solution-for-dummies-still-beats-100-tho/
impl Solution {
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut sorted: Vec<i32> = [nums1, nums2].concat();
    sorted.sort();
    if sorted.len() % 2 == 0 {return (sorted[sorted.len()/2 - 1] + sorted[sorted.len()/2]) as f64 / 2.0}
    return sorted[sorted.len()/2] as f64;
}
}