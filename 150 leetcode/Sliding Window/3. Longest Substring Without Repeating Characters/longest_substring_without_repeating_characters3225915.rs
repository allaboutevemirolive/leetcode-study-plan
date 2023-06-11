// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3225915/rust-with-hashmap-and-manual-slicing/
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut upper = 0;
        let mut lower = 0;
        let mut map:HashMap<char, usize> = HashMap::new();
        let mut max = 0;
        for i in 0..s.len(){
            let c = s.as_bytes()[i] as char;
            if map.contains_key(&c){
                max = std::cmp::max(upper - lower, max);
                let index = map[&c];
                lower = std::cmp::max(index + 1, lower);
            }
            map.insert(c, i);
            upper += 1;
        }
        return std::cmp::max(upper-lower, max) as i32;
    }
}