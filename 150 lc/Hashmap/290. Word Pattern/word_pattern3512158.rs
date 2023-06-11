// https://leetcode.com/problems/word-pattern/solutions/3512158/rust-0ms-array-approach-with-comments/
use std::collections::HashMap;

struct AsciiMap<'a> {
    map: [Option<&'a str>; 26],
    values: Vec<&'a str>
}

impl <'a> AsciiMap<'a> { // the bare bones map API required and a bit more (contains_value)
    fn new() -> Self {
        AsciiMap {map: [None; 26], values: vec![]}
    }

    fn contains_value(&self, val: &str) -> bool {
        self.values.iter().any(|s| s==&val)
    }

    fn get(&mut self, key: &char) -> Option<&str> {
        self.map[*key as usize - 97]
    }

    fn insert<'o> (&mut self, key: char, val: &'o str) 
    where 'o: 'a { // the str slice must outlive this map
        self.map[key as usize - 97] = Some(val);
        self.values.push(val);
    }
}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words = s.split(" ").collect::<Vec<&str>>();
        if pattern.len() != words.len() {return false;}

        let mut map = AsciiMap::new();
        for (c, w) in pattern.chars().zip(words) {
            let contains = map.contains_value(&w);
            
            match map.get(&c) {
                Some(x) if w!=x => {
                    return false; // mapping from c->x cannot map c->w
                },
                None if contains => {
                    return false; // there exists some c'->w cannot map c->w
                }
                _ => {} // anything else is fine
            }
            
            map.insert(c, w);
        }
        true
    }
}