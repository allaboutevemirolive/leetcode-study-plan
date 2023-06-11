impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let nums_size = nums.len() as i32;
        if nums_size == 1 {
            return;
        }
        let k = k % nums_size as i32;
        
        // reverse all array
        for i in 0..nums_size / 2 {
            nums.swap(i as usize, (nums_size - i - 1) as usize);
        }
        
        // reverse first k elements
        for i in 0..k / 2 {
            nums.swap(i as usize, (k - i - 1) as usize);
        }
        
        // reverse last (size - k) elements
        for i in k..k + (nums_size - k) / 2 {
            nums.swap(i as usize, (nums_size - i) as usize - 1 + k as usize);
        }
    }
}