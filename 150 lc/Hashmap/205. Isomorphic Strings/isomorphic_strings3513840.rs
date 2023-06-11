// https://leetcode.com/problems/isomorphic-strings/solutions/3513840/rust-hashmap-array-solutions-easy-to-understand/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.into_bytes(), t.into_bytes());

        // Start with an empty HashMap
        let mut map: HashMap<u8, u8> = HashMap::with_capacity(s.len());

        // Loop through s
        for index in 0..s.len() {
            // If key s[index] is not in the hash map,
            // insert it with a value of t[index]
            let val = map.entry(s[index]).or_insert(t[index]);

            // If the value was not set to t[index],
            // that means one entry is mapped to two different values,
            // and the strings are not isomorphic.
            // For example:
            //
            // s = "egg"          e  g (g)
            //                    |  | \
            // t = "old"          o  l  d
            if *val != t[index] {
                return false;
            }
        }

        // The number of unique values in the hash map
        // should be equal to the number of unique keys.
        // To do this, we collect the keys and values into
        // HashSets and return the comparison of their lengths.
        map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
    }
}