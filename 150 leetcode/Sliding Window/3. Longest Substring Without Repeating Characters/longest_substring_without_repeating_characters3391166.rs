// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3391166/rust-quadratic-loop-with-hash-table-of-chars/
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut substr: HashSet<char> = HashSet::new();
        let mut begin = 0;
        let mut max = 0;
        let mut whole = false;
        while begin < s.len() && !whole {
            whole = true;
            for c in (&s[begin..]).chars() {
                if !substr.contains(&c) {
                    substr.insert(c);
                    if substr.len() > max {
                        max = substr.len();
                    }
                } else {
                    substr.clear();
                    begin += 1;
                    whole = false;
                    break;
                }
            }
        }
        max as i32
    }
}