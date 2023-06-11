// https://leetcode.com/problems/valid-anagram/solutions/2964200/rust-3ms-hashmap-entry-api/
use std::collections::hash_map::{HashMap, Entry};

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // if strings are of different size, anagram impossible
        if s.len() != t.len() { return false; }

        // frequency of each bytes or characters (as the encoding is ASCII for lower case english letters)
        let mut freq: HashMap<u8, i32> = HashMap::new();

        // store the frequency in string s
        for c in s.as_bytes() {
            *freq.entry(*c).or_insert(0) += 1;
        }

        // check there are enough letters in s to obtain t
        for c in t.as_bytes() {
            match freq.entry(*c) {
                Entry::Occupied(ref entry) if *entry.get() < 1 => return false,
                Entry::Occupied(mut entry) => *entry.get_mut() -= 1,
                Entry::Vacant(_) => return false,
            }
        }
        
        true
    }
}