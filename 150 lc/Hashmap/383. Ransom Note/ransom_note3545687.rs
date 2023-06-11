// https://leetcode.com/problems/ransom-note/solutions/3545687/rust-hashmap-entry-api-values-all-check/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;
        let mut letters = HashMap::new();
        for letter in magazine.chars() {
            letters.entry(letter).and_modify(|count| *count += 1).or_insert(1);
        }
        for letter in ransom_note.chars() {
            letters.entry(letter).and_modify(|count| *count -= 1).or_insert(-1);
        }
        letters.values().all(|count| count >= &0)
    }
}