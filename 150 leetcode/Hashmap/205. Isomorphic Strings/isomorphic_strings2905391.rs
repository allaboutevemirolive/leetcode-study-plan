// https://leetcode.com/problems/isomorphic-strings/solutions/2905391/rust-beats-100/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());

        // map s -> t
        let mut map: HashMap<u8, u8> = HashMap::new();
        
        for i in 0..s_bytes.len() {
            let v = map.entry(s_bytes[i]).or_insert(t_bytes[i]);

            // The mapping for char s[i] already exists but is different than t[i]
            if *v != t_bytes[i] {
                return false;
            }
        }

        // Check if multiple chars are mapped to the same char: the number of unique keys
        // and values will be the same if it's one-to-one mapping.
        map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
    }
}