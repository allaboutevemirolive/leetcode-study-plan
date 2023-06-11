```rust
impl Solution {
    pub fn merge(mut nums1: &mut Vec<i32>, m: i32, mut nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums1.append(nums2);
        nums1.sort();
        
    }
}
```
