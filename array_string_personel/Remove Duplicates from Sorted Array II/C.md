```rust
use dedup::DedupBy;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.sort();
        nums.dedup_by(|a, b| *a == *b);
        nums.len() as i32
    }
}
```