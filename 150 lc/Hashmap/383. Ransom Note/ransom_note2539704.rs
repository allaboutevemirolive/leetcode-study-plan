// https://leetcode.com/problems/ransom-note/solutions/2539704/rust-solution-with-hashmap-and-their-methods-if-let/
use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, mut magazine: String) -> bool {
        let mut hash_magazine: HashMap<char, usize> = HashMap::new();
        
        for c in magazine.chars() {
            hash_magazine.entry(c).and_modify(|counter| *counter +=1).or_insert(1);
        }
        for c in ransom_note.chars() {
            if let Some(count) = hash_magazine.get_mut(&c) {
                if *count == 0 {
                    return false;
                }
                *count -=1;
            } else {
                return false;
            }
        }
        true
    }
}