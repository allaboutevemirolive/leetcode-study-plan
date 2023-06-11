// https://leetcode.com/problems/group-anagrams/solutions/3557032/simple-rust-with-hashmap/
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        
        let mut anagrams = HashMap::<String, Vec<String>>::new();

        for s in strs.iter() {
            // Use the stringified byte array as a key (since all anagrams will hash to this!)
            let mut byte_vec = s.as_bytes().to_vec();
            byte_vec.sort();
            let sorted = String::from_utf8(byte_vec.to_vec()).unwrap();

            anagrams.entry(sorted).and_modify(|vec| vec.push(s.to_string())).or_insert([s.clone()].to_vec());
        }

        anagrams.into_values().collect()
    }
}