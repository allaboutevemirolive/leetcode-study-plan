// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3200998/rust-implementation/
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;

        let mut set: HashSet<u8> = HashSet::new();

        let mut i = 0;
        let mut j = 0;

        let n = s.len();
        let s_bytes = s.as_bytes();

        while j < n {
            let current = s_bytes[j];

            if set.contains(&current) {
                while s_bytes[i] != current {
                    set.remove(&s_bytes[i]);
                    i += 1;
                }
                i += 1;
            }

            set.insert(current);
            max_length = max_length.max((j - i + 1) as i32);
            j += 1;
        }

        max_length
    }
}