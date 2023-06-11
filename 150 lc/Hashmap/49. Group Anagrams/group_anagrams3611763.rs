// https://leetcode.com/problems/group-anagrams/solutions/3611763/rust-0ms-beats-100-solution-prime-multiplication/
use std::collections::*;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let primes = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101];
        let mut group = HashMap::new();
        for str in strs {
            let multiple = str.chars().fold(1u128, |mut acc, ch| {
                acc *= primes[ch as usize - 'a' as usize];
                acc
            });
            group.entry(multiple)
                .and_modify(|e: &mut Vec<String>| e.push(str.clone()))
                .or_insert(vec![str]);
        }
        group.into_iter().map(|e| e.1).collect()
    }
}