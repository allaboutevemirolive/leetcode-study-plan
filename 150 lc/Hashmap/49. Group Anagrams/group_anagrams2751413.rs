// https://leetcode.com/problems/group-anagrams/solutions/2751413/rust-hashmap-functional-style-with-comments/
use std::collections::HashMap;

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter().fold(HashMap::<[u8; N_LETTERS], Vec<String>>::new(), |mut map, s| {
            let freqs = s.bytes().map(|b| (b - b'a') as usize).fold([0; N_LETTERS], |mut freqs, bin| {
                freqs[bin] += 1;
                freqs
            });
            map.entry(freqs).or_default().push(s);
            map
        }).into_values().collect()
    }
}