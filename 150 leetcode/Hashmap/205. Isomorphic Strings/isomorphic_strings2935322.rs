// https://leetcode.com/problems/isomorphic-strings/solutions/2935322/rust-0ms-100-early-termination-array-not-hashmap/
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_map = [0; 128]; let mut s_id = 1;
        let mut t_map = [0; 128]; let mut t_id = 1;
        for (&a,&b) in s.into_bytes().iter().zip(t.into_bytes().iter()) {
            let (a, b) = (a as usize, b as usize);
            if s_map[a] == 0 { s_map[a] = s_id; s_id += 1; }
            if t_map[b] == 0 { t_map[b] = t_id; t_id += 1; }
            if s_map[a] != t_map[b] { return false; }
        }
        true
    }
}