// https://leetcode.com/problems/valid-anagram/solutions/3503330/rust-fixed-array-fast-simple/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut sa = [0;26];
        let mut ta = [0;26];

        for c in s.as_bytes() {
            sa[(c - b'a') as usize] += 1;
        }

        for c in t.as_bytes() {
            ta[(c - b'a') as usize] += 1;
        }

        sa == ta
    }
}