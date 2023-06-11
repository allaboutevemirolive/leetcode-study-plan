// https://leetcode.com/problems/isomorphic-strings/solutions/3293422/rust-two-maps-solution/
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut s_to_t: HashMap<u8, u8> = HashMap::new();
        let mut t_to_s: HashMap<u8, u8> = HashMap::new();
        for i in 0..s.len() {
            let (s_c, t_c) = (s[i], t[i]);
            if let Some(t_char) = s_to_t.get(&s_c) {
                if *t_char != t_c {
                    return false;
                }
                // Check if t_c is already mapped to another letter!
            } else if t_to_s.get(&t_c).is_some() {
                return false;
            }
            s_to_t.insert(s_c, t_c);
            t_to_s.insert(t_c, s_c);
        }
        true
    }
}