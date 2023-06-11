```rust
impl Solution {
    // [3, 0, 6, 1, 5]
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        // [3,0,6,1,5] => 3
        // [10,8,5,4,3,3,2,1] => 4
        let mut i = 0;
        let h = citations.len();
        // h = 5
        while i < h && citations[citations.len() - 1 - i] > i as i32 {
            // println!("i: {}, citations[citations.len() - 1 - i]: {:?}", i, citations[citations.len() - 1 - i]);
            i += 1;
            // println!("i: {}", i);
        }

        // i: 0, citations[citations.len() - 1 - i]: 6
        // i: 1
        // i: 1, citations[citations.len() - 1 - i]: 5
        // i: 2
        // i: 2, citations[citations.len() - 1 - i]: 3
        // i: 3


        i as i32
    }
}
```