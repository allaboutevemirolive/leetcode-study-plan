// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3032646/rust-using-hashmap-3-ms-2-2-mb/
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut index_map: HashMap<char, i32> = HashMap::new();
        let mut max_len = 0;
        let mut start = 0;
        let mut curr = 0;

        s.chars().for_each(|c| {
            index_map.entry(c).and_modify(|prev| {
                let len = curr - start;
                if len > max_len {
                    max_len = len;
                }
                if *prev >= start {
                    start = *prev + 1;
                }
                *prev = curr;
            }).or_insert(curr);
            curr += 1;
        });
        curr -= start;
        if curr > max_len { curr } else { max_len }
    }
}
