// https://leetcode.com/problems/word-pattern/solutions/2986259/rust-simple-solution-hashset-hashmap/
use std::collections::{HashMap, HashSet, hash_map::Entry};
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if s.split(' ').collect::<Vec<&str>>().len() != pattern.len() {
            return false;
        }
        let mut matches: HashMap<char, &str> = HashMap::with_capacity(pattern.len());
        let mut found: HashSet<&str> = HashSet::with_capacity(pattern.len());
        let words = s.split(' ');
        for (i, word) in words.enumerate() {
            let c = pattern.chars().nth(i).unwrap();
            match matches.entry(c) {
                Entry::Occupied(entry) if *entry.get() != word => return false,
                Entry::Occupied(_) => (),
                Entry::Vacant(_) => {
                    if !found.get(word).is_none() {
                        return false;
                    }
                    found.insert(word);
                    matches.insert(c, word);
                }
            }
        }
        true
    }
}