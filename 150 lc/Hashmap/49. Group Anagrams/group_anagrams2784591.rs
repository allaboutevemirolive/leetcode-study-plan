// https://leetcode.com/problems/group-anagrams/solutions/2784591/rust-hashmaps-solution/
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();
        let mut result = vec!();
        for s in strs{
            let mut chars: Vec<_> = s.chars().collect();
            chars.sort();
            hm.entry(chars.into_iter().collect::<String>()).or_insert(vec!()).push(s);
        }
        for (k, v) in hm{
            result.push(v);
        }
        result
    }
}