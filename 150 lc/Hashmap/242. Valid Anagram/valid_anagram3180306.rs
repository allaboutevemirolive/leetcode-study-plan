// https://leetcode.com/problems/valid-anagram/solutions/3180306/rust-o-n-functional-solution-3-lines-of-code/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let a = s.chars().fold([0; 26], |mut acc, x| {acc[(x as u8 - 97) as usize] += 1; acc});
        let b = t.chars().fold([0; 26], |mut acc, x| {acc[(x as u8 - 97) as usize] += 1; acc});
        a.iter().zip(b).fold(true, |acc, (i, j)| acc && (*i == j))
    }
}