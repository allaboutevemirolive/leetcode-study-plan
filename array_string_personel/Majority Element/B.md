```rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut majority_el = 0;

        // Example : [3,2,3] or [2,2,1,1,1,2,2]
        for i in nums {
            if count == 0 {
                majority_el = i;
            }
            // Compare previous majority_element with current element
            // If it same, increment the counter
            // If it different, decrement the counter
            if majority_el == i {
                count += 1;
            } else {
                count -= 1;
            }
        }
        
        majority_el
    }
}
```