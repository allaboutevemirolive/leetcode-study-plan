// https://leetcode.com/problems/isomorphic-strings/solutions/3578006/hashtable-0ms/
use std::collections::hash_map::HashMap;

impl Solution {
    #[inline(always)] // no reason not to here
    pub fn is_isomorphic(lhs: String, rhs: String) -> bool {
        debug_assert_eq!(lhs.len(), rhs.len());
        // a foolish assumption in practice but it's in the problem spec and I want speed so ^

        let mut map = HashMap::<char, char>::with_capacity(128);
        let mut values = Vec::with_capacity(128);

        for (lhs, rhs) in lhs.chars().zip(rhs.chars()) {
            match map.insert(lhs, rhs) { // this maps lhs to rhs, and returns the old value (or None)
                Some(v) if v == rhs => continue, // already checked it, next!
                Some(_) => return false, // can't map the chars 1:2 and be isomorphic
                None => () // need to check if value is duplicate so I'm not mapping 2:1
            }
            if values.contains(&rhs) {
                return false // maping two keys to one value, not isomorphic
            }
            values.push(rhs);
        }
        true // the mapping passed all checks therefore there is an isomorphic mapping of lhs and rhs
    }
}