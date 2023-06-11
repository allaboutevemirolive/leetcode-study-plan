// https://leetcode.com/problems/group-anagrams/solutions/3043203/rust-linear-time-in-character-count-constant-space-hash-no-sorting/
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, hash_map::DefaultHasher};

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn get_anagram_key(s: &str) -> u64 {
            let mut hasher = DefaultHasher::new();
            let mut char_counts : [usize;26] = [0;26];
            s.chars().for_each(|c| char_counts[(c as u8 - b'a') as usize] += 1);
            Hash::hash_slice(&char_counts, &mut hasher);
            hasher.finish()
        }

        let mut anagram_map : HashMap<u64, Vec<String>> = HashMap::new();
        strs.iter().for_each(|s| {
            let k = get_anagram_key(&s[..]);
            anagram_map.entry(k).or_insert(vec![]).push(s.to_string());
        });
        anagram_map.into_iter().map(|(k, v)| v).collect::<Vec<Vec<String>>>()
    }
}