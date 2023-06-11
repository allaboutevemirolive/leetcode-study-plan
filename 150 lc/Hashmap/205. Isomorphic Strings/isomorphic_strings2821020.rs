// https://leetcode.com/problems/isomorphic-strings/solutions/2821020/rust-solution/
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut linked_chars: HashMap<char, char> = HashMap::new();
        let mut uniq_chars: Vec<char> = vec![];

        let mut t_chars = t.chars();

        for s_char in s.chars() {
            let t_char = t_chars.next().unwrap();

            let present = linked_chars.insert(s_char, t_char);
            if let Some(val) = present {
                if val == t_char {
                    continue;
                } else {
                    return false;
                }
            } else {
                if uniq_chars.contains(&t_char) {
                    return false;
                }
                uniq_chars.push(t_char);
            }
        }
        
        return true;
    }
}