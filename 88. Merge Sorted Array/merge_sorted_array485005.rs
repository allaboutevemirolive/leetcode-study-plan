// https://leetcode.com/problems/merge-sorted-array/solutions/485005/rust-with-binary-search/
	pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if m == 0 { *nums1 = nums2.clone(); return () } // handle special case of empty first vec
        
        for i in 0..n as usize {
            let target = nums2[i];
            
            let insertion_point = (Solution::search_insert(&nums1[0..m as usize + i], target)); // perform binary search for the proper insertion point of the target
            
            nums1.insert(insertion_point as usize, target); // insert target in nums1
            
            nums1.pop(); // remove the stupid trailing zeroes filling nums1
            
            // I hope someone can explain why this exercise is set up with those unneeded zeroes. My assumption is that the exercise originated in a language that 
            // cannot initialize an empty array/vec/whathaveyou and the port to rust just kept it. If there is some other concept I'm missing I'd love to hear it.
            
        }
    }
    
    fn search_insert(nums: &[i32], target: i32) -> i32 {
        let mut high = (nums.len()-1);
        
        let mut low = 0;
        
        while low <= high {
            let mid = low + (high-low)/2;
        
            match nums[mid] {                
                x if x <= target => low = mid + 1,
                x if x > target => { if mid == 0 { return low as i32 } else { high = mid - 1 } },
                _ => continue
            }
            
        }
        low as i32
    }