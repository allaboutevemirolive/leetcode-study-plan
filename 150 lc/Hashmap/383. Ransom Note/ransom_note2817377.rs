// https://leetcode.com/problems/ransom-note/solutions/2817377/rust/
use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut r_map: HashMap<char, u32> = Default::default();
        let mut m_map: HashMap<char, u32> = Default::default();
        for ch in ransom_note.chars() {
            *r_map.entry(ch).or_insert(0) += 1
        }
        for ch in magazine.chars() {
            *m_map.entry(ch).or_insert(0) += 1
        }
        for (key, value) in &r_map {
            if (m_map.get(key).unwrap_or(&0) < value) {
                return false;
            }
        }
        true
    }
}