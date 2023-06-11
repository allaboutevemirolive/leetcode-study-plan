// https://leetcode.com/problems/word-pattern/solutions/2979344/rust-one-hash-map-one-liner-with-comments/
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
enum Entry<'a> {
    Letter(u8),
    Word(&'a str),
}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        pattern.bytes().map(|l| Some(l)).chain(std::iter::repeat(None))
        .zip(s.split_whitespace().map(|w| Some(w)).chain(std::iter::repeat(None)))
        .enumerate()
        .scan(HashMap::<Entry, usize>::new(), |map, (index, (letter_opt, word_opt))| 
            match (letter_opt, word_opt) {
                (None, None) => return None,
                (Some(letter), Some(word)) => Some(*map.entry(Entry::Letter(letter)).or_insert(index) == *map.entry(Entry::Word(word)).or_insert(index)),
                _ => Some(false),
            }
        ).all(|ok| ok)
    }
}