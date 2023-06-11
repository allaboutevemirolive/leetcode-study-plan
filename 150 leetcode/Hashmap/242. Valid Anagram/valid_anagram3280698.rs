// https://leetcode.com/problems/valid-anagram/solutions/3280698/rust-solution-based-on-fixed-length-array/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut counts: [u32; 26] = [0; 26];

        for c in s.chars() {
            counts[c as usize - 'a' as usize] += 1;
        }

        for c in t.chars() {
            counts[c as usize - 'a' as usize] -= 1;
        }

        !counts.iter().any(|&x| x != 0)
    }
}