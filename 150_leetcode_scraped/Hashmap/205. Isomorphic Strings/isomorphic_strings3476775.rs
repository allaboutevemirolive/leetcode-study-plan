// https://leetcode.com/problems/isomorphic-strings/solutions/3476775/detecting-isomorphic-strings-in-typescript-and-rust/
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (mut map_s, mut map_t) = (HashMap::new(), HashMap::new());

        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        for i in 0..s_chars.len() {
            let (c1, c2) = (s_chars[i], t_chars[i]);

            if let Some(prev) = map_s.get(&c1) {
                if *prev != c2 {
                    return false;
                }
            }

            if let Some(prev) = map_t.get(&c2) {
                if *prev != c1 {
                    return false;
                }
            }
            
            map_s.insert(c1, c2);
            map_t.insert(c2, c1);
        }
        
        true
    }
}
