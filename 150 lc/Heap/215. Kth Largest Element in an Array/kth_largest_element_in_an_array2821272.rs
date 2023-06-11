// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/2821272/rust-linear-space-and-time-complexity-runtime-with-bucket-sort/
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for num in nums.iter() {
            if *num < min {
                min = *num;
            }

            if *num > max {
                max = *num;
            }
        }

        let offset = max - min + 1;
        let mut bucket = vec![0; offset as usize];

        for num in nums.iter() {
            let i = *num - min; 
            bucket[i as usize] += 1;
        }

        let mut mut_k = k; 
        let mut i = (offset-1) as usize; 

        while (i >= 0) {
            mut_k = mut_k - (bucket[i] as i32);
            if mut_k < 1 {
                break;
            }

            i -= 1;
        }

        (i as i32) + min
    }
}