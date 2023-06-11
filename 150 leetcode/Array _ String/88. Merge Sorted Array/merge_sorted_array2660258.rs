// https://leetcode.com/problems/merge-sorted-array/solutions/2660258/rust-100-runtime-and-mem-using-mem-swap/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

        if m == 0 && n != 0 { std::mem::swap(nums1, nums2)}
        else{
            let mut n2v = nums2.into_iter();    
            for v in nums1.iter_mut() {
                if *v == 0 {
                    let mut rv = &mut n2v.next();
                    match &mut rv {
                        Some(r) => {std::mem::swap(v, r);},
                        None => break
                    }
                    
                }
            }    
        }
        nums1.sort()
    }
}