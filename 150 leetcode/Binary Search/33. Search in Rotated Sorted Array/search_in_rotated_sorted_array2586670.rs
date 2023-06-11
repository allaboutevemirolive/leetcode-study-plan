// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2586670/rust-recursive-search-lightning-fast-0ms/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn bin_search_target_logn(N: &[i32], target:i32) -> i32{
            let length = N.len();
            if length <= 0 {return -1}
            let mid = if length == 1 {0} else {length / 2};
            if N[mid] == target { return mid as i32 }
            if N[0] < N[mid] {
                if target <= N[mid] && target >= N[0] { return bin_search_target_logn(&N[0..mid], target)}
                else { 
                    let m = bin_search_target_logn(&N[mid+1..length], target); 
                    if m > -1 {return m + mid as i32 + 1} 
                    else {return -1}
                }
            }
            else{
                if target <= N[length -1] && target >= N[mid] {
                    let m = bin_search_target_logn(&N[mid+1..length], target); 
                    if m > -1 {return m + mid as i32 + 1} 
                    else {return -1}
                } 
                else { return bin_search_target_logn(&N[0..mid], target)}
            }
            -1
        }
        bin_search_target_logn(&nums, target)
    }
}