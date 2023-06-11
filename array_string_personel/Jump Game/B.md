```rust
impl Solution {
    // [2,3,1,1,4]
    // [3,2,1,0,4]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        
        let mut max = 0;
        for (i, &n) in nums.iter().enumerate() {
            if i > max {
                return false;
            }
            max = max.max(i + n as usize);
        }
        true
    }
}
```