// https://leetcode.com/problems/valid-anagram/solutions/3180537/rust-simple-solution-1-ms/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }

        let mut letters = [0_i32; 26];

        s.bytes().zip(t.bytes()).for_each(|(u, v)| {
            letters[(u - b'a') as usize] += 1;
            letters[(v - b'a') as usize] -= 1;
        });
        
        for cnt in letters {
            if cnt != 0 { return false; }
        }
        true
    }
}