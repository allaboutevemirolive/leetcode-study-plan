// https://leetcode.com/problems/word-pattern/solutions/3087019/rust-o-n-track-emergence-order-and-compare-positions/
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words = s.split(' ').collect::<Vec<&str>>();
        if pattern.len() != words.len() { return false; }

        let mut letEmeOrd:HashMap<char,usize> = HashMap::new();
        let mut wordEmeOrd:HashMap<&str,usize> = HashMap::new();
        let (mut lEmeOrdPos,mut wEmeOrdPos) = (vec![0;pattern.len()],vec![0;words.len()]);

        for (i,c) in pattern.chars().enumerate() {
            if !letEmeOrd.contains_key(&c) { letEmeOrd.insert(c,letEmeOrd.len()); }
            lEmeOrdPos[i] = *letEmeOrd.get(&c).unwrap();
        }
        for (i,w) in words.iter().enumerate() {
            if !wordEmeOrd.contains_key(w) { wordEmeOrd.insert(w,wordEmeOrd.len()); }
            wEmeOrdPos[i] = *wordEmeOrd.get(w).unwrap();
        }

        lEmeOrdPos==wEmeOrdPos
    }
}