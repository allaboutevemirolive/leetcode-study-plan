// https://leetcode.com/problems/ransom-note/solutions/2674044/rust-hashmap/
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut passed = true;
        let mut map = HashMap::new();

        for c in magazine.chars() {
            map.entry(c).and_modify(|entry| *entry += 1).or_insert(1);
        }

        for c in ransom_note.chars() {
            if map.contains_key(&c) {
                map.entry(c).and_modify(|entry| {
                    if *entry < 1 {
                        passed = false;
                    }
                    *entry -= 1
                });
            } else {
                passed = false;
            }
        }
        passed
    }
}