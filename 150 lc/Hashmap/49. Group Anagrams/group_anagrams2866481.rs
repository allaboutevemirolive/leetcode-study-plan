// https://leetcode.com/problems/group-anagrams/solutions/2866481/rust-hashmap-with-26-byte-key/
use std::collections::HashMap;

fn word_to_key(word: &String) -> [u8; 26] {
    let mut counts = [0_u8; 26];
    for ch in word.chars() {
        let shift = ch as usize - 'a' as usize;
        counts[shift] += 1;
    }
    counts
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for word in strs {
            map.entry(word_to_key(&word)).or_insert(Vec::new()).push(word);
        }
        map.into_values().collect()
    }
}