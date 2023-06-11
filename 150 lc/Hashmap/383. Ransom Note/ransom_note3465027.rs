// https://leetcode.com/problems/ransom-note/solutions/3465027/rust-intuitive-functional/
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut ma = magazine.chars().collect::<Vec<char>>();
        let mut rn = ransom_note.chars().collect::<Vec<char>>();
        ma.sort();
        rn.sort();
        ma.dedup();
        rn.dedup();

        let ma_counts = ma.iter()
            .map(|c| (c, magazine.chars().filter(|&i|i == *c).count()))
            .collect::<HashMap<&char, usize>>();
        
        let rn_counts = rn.iter()
            .map(|c| (c, ransom_note.chars().filter(|&i|i == *c).count()))
            .collect::<HashMap<&char, usize>>();

        println!("{:?}", ma_counts);
        println!("{:?}", rn_counts);
        rn_counts.iter().all(|(key, value)| ma_counts.get(key) >= Some(value))
    }
}