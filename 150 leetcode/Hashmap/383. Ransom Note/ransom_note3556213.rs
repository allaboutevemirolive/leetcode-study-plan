// https://leetcode.com/problems/ransom-note/solutions/3556213/rust-using-entry-api/
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters = HashMap::new();

        for l in magazine.chars() {
            letters.entry(l).and_modify(|v| *v += 1).or_insert(1);
        }

        for l in ransom_note.chars() {
            let value = letters.entry(l).and_modify(|v| *v -= 1).or_insert(-1);

            if *value < 0 {
                return false;
            }
        }

        return true;
    }
}