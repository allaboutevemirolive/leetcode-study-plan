// https://leetcode.com/problems/ransom-note/solutions/2585016/rust-hashmap-occupied-vacant/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let mut available = magazine.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        ransom_note
            .chars()
            .all(|c| match available.entry(c).and_modify(|e| *e -= 1) {
                Entry::Occupied(e) => e.get().ge(&0),
                Entry::Vacant(_) => false,
            })
    }
}