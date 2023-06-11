// https://leetcode.com/problems/isomorphic-strings/solutions/3390449/rust-solution/
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        fn to_map(s: String) -> (HashMap<i32, i32>, Vec<char>) {
            (HashMap::new(), s.chars().collect())
        }
        let (mut m1, s) = to_map(s);
        let (mut m2, t) = to_map(t);
        for i in 0..t.len() {
            let c1 = s[i] as i32;
            let c2 = t[i] as i32;

            if !m1.contains_key(&c1) && !m2.contains_key(&c2) {
                m1.insert(c1, c2);
                m2.insert(c2, c1);
            }
            if !(*m1.get(&c1).unwrap_or(&c1) == c2 && *m2.get(&c2).unwrap_or(&c2) == c1) {
                return false;
            }
        }
        true    
    }
}